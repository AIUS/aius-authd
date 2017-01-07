use std::{fmt, error};
use toml;
use serde;
use serde::{Deserialize, Serialize};

/// Holds the web server config
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16
}

/// Holds the Redis connection config
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    pub uri: String
}

/// Holds the LDAP connection config
#[derive(Debug, Serialize, Deserialize)]
pub struct LdapConfig {
    pub uri: String,
    pub user: String,
    pub pass: String,
    pub base_dn: String,
}

/// A structure that holds the application config
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub redis: RedisConfig,
    #[serde(default)]
    pub ldap: LdapConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig { port: 8080 }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig { uri: String::from("redis://127.0.0.1/") }
    }
}

impl Default for LdapConfig {
    fn default() -> Self {
        LdapConfig {
            uri: String::from("ldap://127.0.0.1:389"),
            user: String::new(),
            pass: String::new(),
            base_dn: String::new(),
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

impl Config {
    /// Loads a config from a string (toml)
    ///
    /// # Example
    ///
    /// ```rust
    /// Config::load(r#"
    ///     [server]
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
    /// port = 8080
    /// */
    /// ```
    pub fn save(self) -> Result<String, <toml::Encoder as serde::Serializer>::Error> {
        let mut e = toml::Encoder::new();
        try!(self.serialize(&mut e));
        Ok(toml::Value::Table(e.toml).to_string())
    }
}
