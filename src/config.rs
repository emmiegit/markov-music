/*
 * config.rs
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

use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::path::{Path, PathBuf};
use utils::HOME_DIR;
use {toml, Result};

lazy_static! {
    static ref DEFAULT_CONFIG_PATH: PathBuf = {
        let mut dir = match env::var_os("XDG_CONFIG_HOME") {
            Some(dir) => PathBuf::from(dir),
            None => {
                let mut dir = HOME_DIR.clone();
                dir.push(".config");
                dir
            },
        };

        dir.push("markov-music/config.toml");
        dir
    };
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub storage_file: PathBuf,
    pub ipv4: bool,
    pub port: u16,
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;
        let config: Self = toml::from_str(&contents)?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            storage_file: {
                let mut dir = match env::var_os("XDG_DATA_HOME") {
                    Some(dir) => PathBuf::from(dir),
                    None => {
                        let mut dir = HOME_DIR.clone();
                        dir.push(".local/share");
                        dir
                    },
                };

                dir.push("markov-music/chain.db");
                dir
            },
            ipv4: false,
            port: 6600,
        }
    }
}

pub fn parse_args() -> Result<Config> {
    let matches = App::new("Markov Music")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ammon Smith")
        .about(
            "Dynamic music player that chooses your music based on a Markov chain",
        )
        .max_term_width(110)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Use a specific configuration file instead of the default"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("NUMBER")
                .help("Use the given port to connect to mpd"),
        )
        .arg(
            Arg::with_name("ipv4")
                .short("4")
                .long("ipv4")
                .help("Use IPV4 instead of IPv6.")
        )
        .arg(
            Arg::with_name("no-color")
                .long("no-color")
                .help("Disables colors even on terminals that support them"),
        )
        .get_matches();

    let config = match matches.value_of("config") {
        Some(path) => Config::read(Path::new(path))?,
        None if DEFAULT_CONFIG_PATH.is_file() => {
            Config::read(&*DEFAULT_CONFIG_PATH)?
        },
        _ => Config::default(),
    };

    Ok(config)
}
