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

use std::path::{Path, PathBuf};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Tags {
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Song {
    path: PathBuf,
}

impl Song {
    pub fn new(path: &Path) -> Self {
        let path = PathBuf::from(path);
        assert!(path.is_relative());

        Song { path: path }
    }
}
