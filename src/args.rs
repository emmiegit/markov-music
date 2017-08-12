/*
 * args.rs
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

use clap::{App, Arg};
use config::Config;
use error::Error;
use std::path::Path;

pub struct Args {
    pub config: Config,
    pub color: bool,
}

pub fn parse_args() -> Result<Args, Error> {
    let matches = App::new("markov-music")
        .version("0.0.1")
        .about("Music player that chooses music based on a Markov chain")
        .max_term_width(80)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("Use a specific configuration file instead of the default"),
        )
        .arg(
            Arg::with_name("player")
                .short("P")
                .long("player")
                .takes_value(true)
                .value_name("PLAYER")
                .help("Specify which media player to use to play the music"),
        )
        .get_matches();

    let mut args = Args {
        config: match matches.value_of("config") {
            Some(path) => Config::read(Path::new(path))?,
            None => Config::default(),
        },
        color: true,
    };

    if let Some(player) = matches.value_of("player") {
        args.config.player = String::from(player);
    }

    /*
    if let Some(when) = matches.value_of("color") {
    }
    */

    Ok(args)
}
