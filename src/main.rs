extern crate bodyparser;
#[macro_use] extern crate clap;
#[macro_use] extern crate iron;
#[macro_use] extern crate log;
extern crate logger;
extern crate openldap;
extern crate redis;
extern crate router;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;
extern crate toml;
extern crate uuid;

use clap::Arg;
use simplelog::{TermLogger, LogLevelFilter, Config};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::env;

pub mod config;
pub mod server;
pub mod ldap;

fn main() {
    let _ = TermLogger::init(LogLevelFilter::Info, Config::default());

    let args = app_from_crate!()
        .arg(Arg::with_name("config")
             .long("config")
             .short("c")
             .takes_value(true)
             .value_name("FILE")
             .help("Path to the config file"))
        .arg(Arg::with_name("server.address")
             .long("server-address")
             .short("a")
             .takes_value(true)
             .value_name("ADDRESS")
             .help("Address used for the web server"))
        .arg(Arg::with_name("server.port")
             .long("server-port")
             .short("p")
             .takes_value(true)
             .value_name("PORT")
             .help("Port used for the web server"))
        .arg(Arg::with_name("redis.uri")
             .long("redis-uri")
             .takes_value(true)
             .value_name("URI")
             .help("URI used to connect to the redis database"))
        .arg(Arg::with_name("ldap.uri")
             .long("ldap-uri")
             .takes_value(true)
             .value_name("URI")
             .help("URI used to connect to the LDAP server"))
        .arg(Arg::with_name("ldap.user")
             .long("ldap-user")
             .takes_value(true)
             .value_name("USER")
             .help("Username used to authenticate with the LDAP server"))
        .arg(Arg::with_name("ldap.pass")
             .long("ldap-pass")
             .takes_value(true)
             .value_name("PASSWORD")
             .help("Password used to authenticate with the LDAP server"))
        .arg(Arg::with_name("ldap.base_dn")
             .long("ldap-base-dn")
             .takes_value(true)
             .value_name("BASE_DN")
             .help("Base DN to bind when connecting to the LDAP server"))
        .arg(Arg::with_name("ldap.user_pattern")
             .long("ldap-user-pattern")
             .takes_value(true)
             .value_name("PATTERN")
             .help("Pattern used to search users. Ex: `CN=%USER%,OU=people,DC=example,DC=org`"))
        .get_matches();

    let env = env::vars()
        .map(|(k, v)| (k.to_uppercase(), v))
        .filter(|&(ref k, _)| k.starts_with("AUTH_"))
        .collect::<HashMap<String, String>>();

    // @TODO: Error handling when loading the config file
    let config = match args.value_of("config")
        .or(env.get("AUTH_CONFIG").map(|s| s.as_str())) {
        Some(p) => {
            let mut f = File::open(p).unwrap();
            let mut toml = String::new();
            f.read_to_string(&mut toml).unwrap();
            config::Config::load(toml.as_str()).unwrap()
        },
        None => config::Config::default()
    }
    .merge_with_args(args)
    .merge_with_env(env);

    info!("Redis server: {}", config.redis.uri);
    info!("LDAP server: {}", config.ldap.uri);

    // @TODO: Error handling when starting server
    let server = server::start(config.clone(), server::get_handler()).unwrap();
    info!("Listening on {}", server.socket);
}
