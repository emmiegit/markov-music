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

    let args = Args {
        config: config,
        color: match matches.value_of("color") {
            Some(mode) => {
                match mode {
                    "always" => ColorChoice::Always,
                    "auto" => ColorChoice::Auto,
                    "never" => ColorChoice::Never,
                    _ => {
                        return Err(Error::new("Color mode is not one of 'always', 'auto', or 'never'"));
                    }
                }
            }
            None => ColorChoice::Auto,
        }
    };

    Ok(args)
}
