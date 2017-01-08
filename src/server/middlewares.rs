use iron::prelude::*;
use iron::typemap;
use iron::BeforeMiddleware;
use config;

pub struct CurrentConfig;
impl typemap::Key for CurrentConfig {
    type Value = config::Config;
}

pub struct ConfigBeforeMiddleware {
    config: config::Config,
}

impl ConfigBeforeMiddleware {
    pub fn new(config: config::Config) -> Self {
        ConfigBeforeMiddleware { config: config }
    }
}

impl BeforeMiddleware for ConfigBeforeMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<CurrentConfig>(self.config.clone());
        Ok(())
    }
}
