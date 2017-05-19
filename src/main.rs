extern crate clap;
extern crate markov;
extern crate mpv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termcolor;

mod args;
mod config;
mod error;
mod player;

fn main() {
    println!("Hello, world!");
}
