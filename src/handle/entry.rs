/*
 * handle/entry.rs
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

use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub ftype: EntryType,
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        &self.path == &other.path
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
