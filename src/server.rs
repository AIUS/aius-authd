use iron::error::HttpResult;
use iron::Listening;
use iron::typemap;
use iron::status;
use iron::prelude::*;
use serde_json;
use config;

struct CurrentConfig;
impl typemap::Key for CurrentConfig {
    type Value = config::Config;
}

fn handler(req: &mut Request) -> IronResult<Response> {
    // Print the config as a test
    let config = req.extensions.get::<CurrentConfig>().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string_pretty(config).unwrap())))
}

pub fn start(config: config::Config) -> HttpResult<Listening> {
    // Address must be cloned before moved to middleware
    let addr = format!("{}:{}", config.server.address, config.server.port).clone();

    // Middleware chain
    let mut chain = Chain::new(handler);

    // Middleware used to expose the config through the request
    chain.link_before(move |req: &mut Request| {
        req.extensions.insert::<CurrentConfig>(config.clone());
        Ok(())
    });

    // Spin up the server
    Iron::new(chain).http(addr.as_str())
}
