use bodyparser;
use serde_json;
use redis;
use redis::Commands;
use iron::prelude::*;
use iron::status;
use router::Router;
use server::middlewares::CurrentConfig;
use ldap;
use uuid;

fn config_handler(req: &mut Request) -> IronResult<Response> {
    // Print the config as a test
    let config = req.extensions.get::<CurrentConfig>().unwrap();
    Ok(Response::with((status::Ok, itry!(serde_json::to_string_pretty(config)))))
}

fn ping_handler(req: &mut Request) -> IronResult<Response> {
    // Pings the Redis server
    let con = itry!(req.extensions.get::<CurrentConfig>().unwrap().redis.connect());
    let ret: String = redis::cmd("PING").query(&con).unwrap();
    Ok(Response::with((status::Ok, ret)))
}

#[derive(Deserialize, Clone, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    token: String,
}

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let login = iexpect!(itry!(req.get::<bodyparser::Struct<LoginRequest>>()));
    itry!(ldap::login(login.username.as_str(), login.password.as_str(),
                      req.extensions.get::<CurrentConfig>().unwrap().clone().ldap));

    // Generate token
    let uuid = uuid::Uuid::new_v4();

    // Save token
    let redis = itry!(req.extensions.get::<CurrentConfig>().unwrap().redis.connect());
    let _: () = itry!(redis.set_ex(format!("token:{}", uuid.simple()), login.username.as_str(), 3600 * 24 * 7));

    let resp = LoginResponse {
        token: uuid.hyphenated().to_string(),
    };

    Ok(Response::with((status::Ok, itry!(serde_json::to_string_pretty(&resp)))))
}

#[derive(Serialize, Debug)]
struct ValidateResponse {
    username: String,
    scopes: Vec<String>,
}

fn validate_handler(req: &mut Request) -> IronResult<Response> {
    let token = req.extensions.get::<Router>().unwrap().find("token").unwrap();
    let uuid = itry!(uuid::Uuid::parse_str(token));
    let redis = itry!(req.extensions.get::<CurrentConfig>().unwrap().redis.connect());
    let username: String = itry!(redis.get(format!("token:{}", uuid.simple())));

    let resp = ValidateResponse {
        username: username,
        scopes: vec![String::from("bleh")],
    };

    Ok(Response::with((status::Ok, itry!(serde_json::to_string_pretty(&resp)))))
}


fn revoke_handler(req: &mut Request) -> IronResult<Response> {
    let token = req.extensions.get::<Router>().unwrap().find("token").unwrap();
    let uuid = itry!(uuid::Uuid::parse_str(token));
    let redis = itry!(req.extensions.get::<CurrentConfig>().unwrap().redis.connect());
    let _: () = itry!(redis.del(format!("token:{}", uuid.simple())));

    Ok(Response::with((status::Ok, "OK")))
}


pub fn get_handler() -> Router {
    let mut router = Router::new();
    router.get("/config", config_handler, "config");
    router.get("/ping", ping_handler, "ping");
    router.post("/login", login_handler, "login");
    router.get("/validate/:token", validate_handler, "validate");
    router.get("/revoke/:token", revoke_handler, "revoke");
    router
}
