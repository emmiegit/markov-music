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

// Configuration helpers
pub fn read_config(path: &Path) -> Result<Config, Error> {
    let file = File::open(path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

fn default_config() -> Config {
    const DEFAULT_PLAYER: &str = "mpv";
    const MUSIC_DIR_NAME: &str = "Music";
    const MARKOV_FILE_NAME: &str = ".markov_music.json";

    Config {
        player: DEFAULT_PLAYER.to_string(),
        music_dir: {
            let mut pathbuf = env::home_dir().expect("Unable to get home directory");
            pathbuf.push(MUSIC_DIR_NAME);

            pathbuf.as_path().to_str().expect("Unable to convert path to string").to_string()
        },
        markov_storage_file: MARKOV_FILE_NAME.to_string()
    }
}

pub fn read_default_config() -> Config {
    const CONFIG_FILE: &str = "markov-music.json";
    let mut path = env::home_dir().expect("Unable to get home directory");

    // Get configuration directory
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => {
            if val.len() > 0 {
                path.push(val);
            } else {
                path.push(".config");
            }
        },
        Err(_) => {
            path.push(".config");
        }
    }

    // Read "$CONFIG/markov-music/config.json"
    path.push(CONFIG_FILE);
    match read_config(path.as_path()) {
        Ok(cfg) => cfg,
        Err(_) => default_config(),
    }
}
