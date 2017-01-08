use serde_json;
use redis;
use iron::prelude::*;
use iron::status;
use router::Router;
use server::middlewares::CurrentConfig;
use ldap::LdapConn;

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

fn ldap_handler(req: &mut Request) -> IronResult<Response> {
    let _ = itry!(LdapConn::from_config(req.extensions.get::<CurrentConfig>().unwrap().ldap.clone()));
    Ok(Response::with((status::Ok, "Connection success")))
}

pub fn get_handler() -> Router {
    let mut router = Router::new();
    router.get("/config", config_handler, "config");
    router.get("/ping", ping_handler, "ping");
    router.get("/ldap", ldap_handler, "ldap");
    router
}
