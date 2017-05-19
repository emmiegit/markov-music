extern crate clap;
extern crate markov;
extern crate mpv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termcolor;

use args::{Args, parse_args};
use error::Error;
use std::io::Write;
use std::process::exit;

mod args;
mod config;
mod error;
mod player;

#[inline]
fn print_error(err: Error) {
    writeln!(std::io::stderr(), "Error: {}", err).unwrap();
}

fn main() {
    let args: Args;
    match parse_args() {
        Ok(x) => {
            args = x;
        }
        Err(e) => {
            print_error(e);
            exit(1);
        }
    }


}
