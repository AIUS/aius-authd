use serde_json;
use iron::prelude::*;
use iron::status;
use router::Router;
use server::middlewares::CurrentConfig;

fn config_handler(req: &mut Request) -> IronResult<Response> {
    // Print the config as a test
    let config = req.extensions.get::<CurrentConfig>().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string_pretty(config).unwrap())))
}

pub fn get_handler() -> Router {
    let mut router = Router::new();
    router.get("/config", config_handler, "config");
    router
}
