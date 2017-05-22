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
use config::{Config, read_config, read_default_config};
use error::Error;
use std::path::Path;
use termcolor::ColorChoice;

pub struct Args {
    pub config: Config,
    pub color: ColorChoice,
}

pub fn parse_args() -> Result<Args, Error> {
    let matches = App::new("markov-music")
        .version("0.0.1")
        .about("Music player that chooses music based on a Markov chain")
        .max_term_width(80)
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .takes_value(true)
                 .value_name("FILE")
                 .help("Use a specific configuration file instead of the default"))
        .arg(Arg::with_name("player")
                 .short("P")
                 .long("player")
                 .takes_value(true)
                 .value_name("PLAYER")
                 .help("Specify which media player to use to play the music"))
        .arg(Arg::with_name("color")
                 .long("color")
                 .takes_value(true)
                 .value_name("WHEN")
                 .help("Specify when color output is used"))
        .get_matches();

    let mut config = match matches.value_of("config") {
        Some(path) => read_config(Path::new(path))?,
        None => read_default_config(),
    };

    if let Some(player) = matches.value_of("player") {
        config.set_player(player);
    }

    const INVALID_COLOR_MODE: &str = "Color mode is not one of 'always', 'auto', or 'never'";
    let args = Args {
        config: config,
        color: match matches.value_of("color") {
            Some(mode) => {
                match mode {
                    "always" => ColorChoice::Always,
                    "auto" => ColorChoice::Auto,
                    "never" => ColorChoice::Never,
                    _ => {
                        return Err(Error::new(INVALID_COLOR_MODE));
                    }
                }
            }
            None => ColorChoice::Auto,
        },
    };

    Ok(args)
}
