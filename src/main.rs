extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod config;
use config::Config;

fn main() {
    let a = Config::load(r#"
        [redis]
        uri = "bleh"
    "#).unwrap();

    println!("{:?}", a);
    println!("toml:\n{}", a.save().unwrap());
}
