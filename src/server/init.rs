use iron::error::HttpResult;
use iron::Listening;
use iron::Handler;
use iron::prelude::*;
use logger::Logger;

use server::middlewares::{ConfigBeforeMiddleware, ErrorAfterMiddleware};
use config::Config;

pub fn start<H: Handler>(config: Config, handler: H) -> HttpResult<Listening> {
    // Setup the middleware chain
    let mut chain = Chain::new(handler);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_before(ConfigBeforeMiddleware::new(config.clone()));
    chain.link_after(ErrorAfterMiddleware {});
    chain.link_after(logger_after);

    // Spin up the server
    Iron::new(chain)
        .http(format!("{}:{}", config.server.address, config.server.port).as_str())
}
