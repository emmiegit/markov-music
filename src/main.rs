/*
 * main.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017 Ammon Smith
 *
 * markov-music is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * markov-music is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with markov-music.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate clap;
extern crate mpv;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate termcolor;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

use args::{Args, parse_args};
use std::process::exit;

mod args;
mod config;
mod error;
mod markov;
mod player;
mod song;

fn main() {
    let args: Args;
    match parse_args() {
        Ok(x) => {
            args = x;
        }
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
    }


    // TODO args.config
}
