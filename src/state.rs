/*
 * state.rs
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
use markov::Chain;
use rmp_serde::{Serializer, Deserializer};
use serde::{Serialize, Deserialize};
use song::Song;
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub chain: Chain<Song>,
    pub current: Option<Song>,
    pub cursor: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            chain: Chain::new(),
            current: None,
            cursor: 0,
        }
    }

    pub fn read(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        let mut de = Deserializer::new(&file);
        let state = Deserialize::deserialize(&mut de)?;

        Ok(state)
    }

    pub fn write(&self, path: &Path) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut se = Serializer::new(&mut file);
        self.serialize(&mut se)?;

        Ok(())
    }
}
