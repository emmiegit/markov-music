/*
 * handle/entry/iterator.rs
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
use handle::entry::{Entry, Entries};

#[derive(Debug, Hash)]
pub struct EntryIterator<'a> {
    entries: &'a Entries,
    index: usize,
}

impl<'a> EntryIterator<'a> {
    pub fn new(entries: &'a Entries) -> Self {
        EntryIterator {
            entries: entries,
            index: 0,
        }
    }
}

impl<'a> Iterator for EntryIterator<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<&'a Entry> {
        if self.index >= self.entries.len() {
            return None;
        }

        let ent = &self.entries[self.index];
        self.index += 1;
        Some(ent)
    }
}
