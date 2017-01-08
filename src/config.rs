use std::{fmt, error};
use clap;
use redis;
use serde;
use serde::{Deserialize, Serialize};
use toml;

/// Holds the web server config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_address")]
    pub address: String,
    #[serde(default = "default_server_port")]
    pub port: u16
}

/// Holds the Redis connection config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    #[serde(default = "default_redis_uri")]
    pub uri: String
}

impl RedisConfig {
    pub fn connect(&self) -> redis::RedisResult<redis::Connection> {
        let client = try!(redis::Client::open(self.uri.as_str()));
        client.get_connection()
    }
}

/// Holds the LDAP connection config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapConfig {
    #[serde(default = "default_ldap_uri")]
    pub uri: String,
    #[serde(default = "default_ldap_user")]
    pub user: String,
    #[serde(default = "default_ldap_pass")]
    pub pass: String,
    #[serde(default = "default_ldap_base_dn")]
    pub base_dn: String,
}

/// A structure that holds the application config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub redis: RedisConfig,
    #[serde(default)]
    pub ldap: LdapConfig,
}

fn default_server_address() -> String { String::from("localhost") }
fn default_server_port() -> u16 { 8080 }
fn default_redis_uri() -> String { String::from("redis://127.0.0.1/") }
fn default_ldap_uri() -> String { String::from("ldap://127.0.0.1:389") }
fn default_ldap_user() -> String { String::new() }
fn default_ldap_pass() -> String { String::new() }
fn default_ldap_base_dn() -> String { String::new() }

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig { address: default_server_address(), port: default_server_port() }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig { uri: default_redis_uri() }
    }
}

impl Default for LdapConfig {
    fn default() -> Self {
        LdapConfig {
            uri: default_ldap_uri(),
            user: default_ldap_user(),
            pass: default_ldap_pass(),
            base_dn: default_ldap_base_dn(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::default(),
            redis: RedisConfig::default(),
            ldap: LdapConfig::default(),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    DecodeError(toml::DecodeError),
    ParserErrors(Vec<toml::ParserError>),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadError::DecodeError(ref err) => write!(f, "Decode Error: {}", err),
            LoadError::ParserErrors(ref errs) => {
                errs.into_iter().fold(write!(f, "Parse Errors:"), |result, ref error| {
                    result.and(write!(f, "\n  {}", error))
                })
            },
        }
    }
}

impl error::Error for LoadError {
    fn description(&self) -> &str {
        match *self {
            LoadError::DecodeError(ref err) => err.description(),
            LoadError::ParserErrors(ref errs) => match errs.first() {
                Some(ref err) => err.description(),
                None => "ParserErrors",
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LoadError::DecodeError(ref err) => Some(err),
            LoadError::ParserErrors(ref errs) => errs.first().and_then(|ref err| err.cause()),
        }
    }
}

macro_rules! merge_with_arg {
    // Special case for `String`, to avoid `unreachable_patterns`
    ($c:ident.$p1:ident.$p2:ident, String, $m:ident) => {
        if let Some(v) = $m.value_of(concat!(stringify!($p1), '.', stringify!($p2))) {
            $c.$p1.$p2 = String::from(v);
        }
    };
    ($c:ident.$p1:ident.$p2:ident, $t:ty, $m:ident) => {
        if let Some(v) = $m.value_of(concat!(stringify!($p1), '.', stringify!($p2))) {
            match v.parse::<$t>() {
                Ok(val) => $c.$p1.$p2 = val,
                Err(_)  =>
                    ::clap::Error::value_validation_auto(
                        format!("The argument '{}' isn't a valid value", v)).exit(),
            }
        }
    };
}

impl Config {
    /// Loads a config from a string (toml)
    ///
    /// # Example
    ///
    /// ```rust
    /// Config::load(r#"
    ///     [server]
    ///     address = "0.0.0.0"
    ///     port = 1234
    ///
    ///     [redis]
    ///     uri = "redis://127.0.0.1/"
    ///
    ///     [ldap]
    ///     uri = "ldap://127.0.0.1:389
    ///     user = "admin"
    ///     pass = "password"
    ///     base_dn = "DC=example,DC=com"
    /// "#);
    /// ```
    pub fn load(config: &str) -> Result<Self, LoadError> {
        let mut parser = toml::Parser::new(config);
        let value = match parser.parse() {
            Some(v) => v,
            None => return Err(LoadError::ParserErrors(parser.errors)),
        };
        let mut decoder = toml::Decoder::new(toml::Value::Table(value));
        Deserialize::deserialize(&mut decoder).map_err(|e| LoadError::DecodeError(e))
    }

    /// Save the config to string (toml)
    ///
    /// # Example
    ///
    /// ```rust
    /// Config::default().save().unwrap();
    /// /*
    /// [ldap]
    /// base_dn = ""
    /// pass = ""
    /// uri = "ldap://127.0.0.1:389"
    /// user = ""
    ///
    /// [redis]
    /// uri = "redis://127.0.0.1/"
    ///
    /// [server]
    /// address = "localhost"
    /// port = 8080
    /// */
    /// ```
    pub fn save(self) -> Result<String, <toml::Encoder as serde::Serializer>::Error> {
        let mut e = toml::Encoder::new();
        try!(self.serialize(&mut e));
        Ok(toml::Value::Table(e.toml).to_string())
    }

    /// Merge a config with a `clap::ArgMatches`
    pub fn merge_with_args(self, args: clap::ArgMatches) -> Self {
        let mut config = self.clone();
        merge_with_arg!(config.server.address, String, args);
        merge_with_arg!(config.server.port, u16, args);
        merge_with_arg!(config.redis.uri, String, args);
        merge_with_arg!(config.ldap.uri, String, args);
        merge_with_arg!(config.ldap.base_dn, String, args);
        merge_with_arg!(config.ldap.user, String, args);
        merge_with_arg!(config.ldap.pass, String, args);
        config
    }
}
