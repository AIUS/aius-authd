#[macro_use] extern crate clap;
extern crate toml;
extern crate iron;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

pub mod config;
pub mod server;

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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
        .get_matches();

    // @TODO: Error handling when loading the config file
    let config = match args.value_of("config") {
        Some(p) => {
            let mut f = File::open(p).unwrap();
            let mut toml = String::new();
            f.read_to_string(&mut toml).unwrap();
            config::Config::load(toml.as_str()).unwrap()
        },
        None => config::Config::default()
    }.merge_with_args(args);

    // @TODO: Pretty startup info
    println!("{:?}", config);

    // @TODO: Error handling when starting server
    let _ = server::start(config.clone()).unwrap();
}
