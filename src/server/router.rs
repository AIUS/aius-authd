use bodyparser;
use serde_json;
use redis;
use iron::prelude::*;
use iron::status;
use router::Router;
use server::middlewares::CurrentConfig;
use ldap;

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

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let login = iexpect!(itry!(req.get::<bodyparser::Struct<LoginRequest>>()));
    itry!(ldap::login(login.username.as_str(), login.password.as_str(),
                      req.extensions.get::<CurrentConfig>().unwrap().clone().ldap));
    Ok(Response::with((status::Ok, "Login OK")))
}

pub fn get_handler() -> Router {
    let mut router = Router::new();
    router.get("/config", config_handler, "config");
    router.get("/ping", ping_handler, "ping");
    router.post("/login", login_handler, "login");
    router
}
