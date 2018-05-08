/*
 * player/mpv_player.rs
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

use Result;
use config::Config;
use mpd::{self, Idle, Subsystem};
use std::net::IpAddr::*;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

#[derive(Debug)]
pub struct Player {
    conn: mpd::Client,
}

impl Player {
    pub fn new(config: &Config) -> Result<Self> {
        let addr = if config.ipv4 {
            V4(Ipv4Addr::new(127, 0, 0, 1))
        } else {
            V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        };

        let conn = mpd::Client::connect(SocketAddr::new(addr, config.port))?;
        Ok(Player { conn })
    }

    pub fn update(&mut self) -> Result<()> {
        let _ = self.conn.wait(&[Subsystem::Update, Subsystem::Queue])?;

    let _status = self.conn.status()?;
    let _stats = self.conn.stats()?;

        unimplemented!()
    }
}
