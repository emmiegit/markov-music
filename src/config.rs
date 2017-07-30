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

use error::Error;
use serde_json;
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    player: String,
    music_dir: String,
    markov_storage_file: String,
}

impl Config {
    fn default_config() -> Config {
        const DEFAULT_PLAYER: &str = "mpv";
        const MUSIC_DIR_NAME: &str = "music";
        const MARKOV_FILE_NAME: &str = ".markov_music.json";

        Config {
            player: DEFAULT_PLAYER.to_string(),
            music_dir: {
                let mut pathbuf = env::home_dir().expect("Unable to get home directory");
                pathbuf.push(MUSIC_DIR_NAME);

                pathbuf
                    .as_path()
                    .to_str()
                    .expect("Unable to convert path to string")
                    .to_string()
            },
            markov_storage_file: MARKOV_FILE_NAME.to_string(),
        }
    }

    pub fn read(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;

        Ok(config)
    }

    pub fn default() -> Self {
        const CONFIG_HOME: &str = ".config";
        const CONFIG_DIR: &str = "markov-music";
        const CONFIG_FILE: &str = "markov-music.json";
        let mut path = env::home_dir().expect("Unable to get home directory");

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

        // Read "$CONFIG/markov-music/config.json"
        path.push(CONFIG_DIR);
        path.push(CONFIG_FILE);
        match Config::read(path.as_path()) {
            Ok(cfg) => cfg,
            Err(_) => Config::default_config(),
        }
    }

    pub fn set_player(&mut self, player: &str) {
        self.player = player.to_string();
    }

    pub fn get_player(&self) -> &str {
        &self.player
    }

    pub fn get_music_dir(&self) -> &str {
        &self.music_dir
    }

    pub fn get_markov_storage_file(&self) -> &str {
        &self.markov_storage_file
    }
}
