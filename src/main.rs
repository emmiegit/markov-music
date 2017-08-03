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
extern crate termion;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

use args::parse_args;
use markov::Chain;
use player::{MpvPlayer, Player};
use std::env;
use std::path::Path;
use std::process::exit;

mod args;
mod config;
mod error;
mod markov;
mod player;
mod song;
mod ui;

fn get_player(player_name: &str) -> Player {
    match player_name {
        "mpv" => Player::Mpv(MpvPlayer::new()),
        _ => {
            println!("Unknown type of player: '{}'", player_name);
            exit(1);
        }
    }
}

fn main() {
    let config = match parse_args() {
        Ok(x) => x,
        Err(e) => {
            println!("Error parsing arguments: {}", e);
            exit(1);
        }
    };

    let chain = {
        let pathstr = &config.markov_storage_file;
        let path = Path::new(pathstr);
        match Chain::read(path) {
            Ok(x) => x,
            Err(e) => {
                println!("Can't read markov data: {}", e);
                exit(1);
            }
        };
    };

    if let Err(e) = env::set_current_dir(&config.music_dir) {
        println!("Can't switch to music directory '{}': {}",
                 config.music_dir, e);
        exit(1);
    }

    let player = get_player(&config.player);

    // TODO args.config
}
