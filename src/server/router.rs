use serde_json;
use redis;
use iron::prelude::*;
use iron::status;
use router::Router;
use server::middlewares::CurrentConfig;

fn config_handler(req: &mut Request) -> IronResult<Response> {
    // Print the config as a test
    let config = req.extensions.get::<CurrentConfig>().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string_pretty(config).unwrap())))
}

fn ping_handler(req: &mut Request) -> IronResult<Response> {
    // Pings the Redis server
    let con = req.extensions.get::<CurrentConfig>().unwrap().redis.connect().unwrap();
    let ret: String = redis::cmd("PING").query(&con).unwrap();
    Ok(Response::with((status::Ok, ret)))
}

pub fn get_handler() -> Router {
    let mut router = Router::new();
    router.get("/config", config_handler, "config");
    router.get("/ping", ping_handler, "ping");
    router
}
