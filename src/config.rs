/*
 * config.rs
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

use error::{Error, ErrorCause};
use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use utils::HOME_DIR_PATH;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub music_dir: String,
    pub storage_file: String,
    pub seek_seconds: f32,
    pub volume_step: i32,
}

impl Config {
    fn default_config() -> Config {
        const MUSIC_DIR_NAME: &str = "music";
        const MARKOV_FILE_NAME: &str = ".markov_music.json";

        Config {
            music_dir: {
                let mut path = HOME_DIR_PATH.clone();
                path.push(MUSIC_DIR_NAME);
                path.as_path()
                    .to_str()
                    .expect("Unable to convert path to string")
                    .to_string()
            },
            storage_file: MARKOV_FILE_NAME.to_string(),
            seek_seconds: 0.2,
            volume_step: 1,
        }
    }

    pub fn read(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;

        if !config.seek_seconds.is_finite() {
            Err(Error::new(
                "Seek seconds is not a real number",
                ErrorCause::NoCause(),
            ))
        } else if config.seek_seconds == 0.0 {
            Err(Error::new("Seek seconds is zero", ErrorCause::NoCause()))
        } else if config.volume_step < 0 {
            Err(Error::new("Volume step is negative", ErrorCause::NoCause()))
        } else if config.volume_step > 100 {
            Err(Error::new(
                "Volume step is greater than 100",
                ErrorCause::NoCause(),
            ))
        } else {
            Ok(config)
        }
    }

    pub fn default() -> Result<Self, Error> {
        const CONFIG_HOME: &str = ".config";
        const CONFIG_DIR: &str = "markov-music";
        const CONFIG_FILE: &str = "config.toml";
        let mut path = HOME_DIR_PATH.clone();

        // Get configuration directory
        match env::var("XDG_CONFIG_HOME") {
            Ok(val) => {
                if val.len() > 0 {
                    path.push(val);
                } else {
                    path.push(CONFIG_HOME);
                }
            }
            Err(_) => {
                path.push(CONFIG_HOME);
            }
        }

        // Read "$CONFIG/markov-music/config.toml"
        path.push(CONFIG_DIR);
        path.push(CONFIG_FILE);
        match Config::read(path.as_path()) {
            Ok(cfg) => Ok(cfg),
            Err(e) => {
                if !path.is_file() {
                    Ok(Config::default_config())
                } else {
                    Err(e)
                }
            }
        }
    }
}
