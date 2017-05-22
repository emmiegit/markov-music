/*
 * selector.rs
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

use config::Config;
use error::Error;
use markov::Chain;
use player::MpvPlayer;
use player::Player;
use song::Song;

pub struct Selector {
    player: Player,
    chain: Chain<Song>,
}

impl Selector {
    pub fn new(cfg: &Config) -> Result<Self, Error> {
        let mut player = match cfg.get_player() {
            "mpv" => Player::Mpv(MpvPlayer::new()),
            _ => {
                return Err(Error::new("Unknown player type"));
            }
        };

        Ok(Selector {
               player: player,
               chain: Chain::new(),
           })
    }
}
