/*
 * song.rs
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

use std::path::Path;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Song {
    path: String,
}

impl Song {
    pub fn new(pathstr: &str) -> Self {
        let path = Path::new(pathstr);
        assert!(path.is_relative());

        Song { path: String::from(pathstr) }
    }

    pub fn get_str(&self) -> &str {
        &self.path
    }

    pub fn get_path(&self) -> &Path {
        Path::new(&self.path)
    }
}
