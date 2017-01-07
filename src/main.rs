//! Bleh.

extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod config;

fn main() {
    let a = config::Config::load(r#"
        [redis]
        uri = "bleh"
    "#).unwrap();

    println!("{:?}", a);
    println!("toml:\n{}", a.save().unwrap());
}
