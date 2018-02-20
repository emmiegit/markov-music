/*
 * main.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017-2018 Ammon Smith
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

#![deny(missing_debug_implementations)]

extern crate clap;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate mpd;
extern crate rand;

#[macro_use]
extern crate serde;
extern crate simple_logging;
extern crate toml;

mod config;
mod database;
mod error;
mod handle;
mod logging;
mod markov;
mod player;
mod utils;

use config::{parse_args, Config};
use handle::Handle;
use std::{env, result};
use std::process::exit;
use utils::HOME_DIR;

pub use error::{Error, StdError};

pub type StdResult<T, E> = result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

fn main() {
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            exit(1);
        }
    };

    logging::setup();

    if let Err(e) = main_loop(config) {
        eprintln!("Error in main loop: {}", e);
        exit(1);
    }
}

fn main_loop(config: Config) -> Result<()> {
    let mut handle = Handle::new(config)?;

    loop {
        handle.wait()?;
        // TODO
    }
}
