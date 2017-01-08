use iron::error::HttpResult;
use iron::Listening;
use iron::Handler;
use iron::prelude::*;

use server::middlewares::{ConfigBeforeMiddleware, ErrorAfterMiddleware};
use config::Config;

pub fn start<H: Handler>(config: Config, handler: H) -> HttpResult<Listening> {
    // Setup the middleware chain
    let mut chain = Chain::new(handler);
    chain.link_before(ConfigBeforeMiddleware::new(config.clone()));
    chain.link_after(ErrorAfterMiddleware {});

    // Spin up the server
    Iron::new(chain)
        .http(format!("{}:{}", config.server.address, config.server.port).as_str())
}
