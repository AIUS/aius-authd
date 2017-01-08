use iron::prelude::*;
use iron::typemap;
use iron::status;
use iron::{AfterMiddleware, BeforeMiddleware};
use redis::RedisError;
use openldap::errors::LDAPError;
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

pub struct ErrorAfterMiddleware;
impl AfterMiddleware for ErrorAfterMiddleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<RedisError>() {
            return Ok(Response::with((status::ServiceUnavailable, "Redis Error")))
        } else if err.error.is::<LDAPError>() {
            return Ok(Response::with((status::ServiceUnavailable, "LDAP Error")))
        } else {
            Err(err)
        }
    }
}
