/*
 * handle/cursor.rs
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
use handle::entry::{Entry, EntryType, Entries, EntryIterator};
use std::cmp;
use std::path::{Path, PathBuf};
use utils;

#[derive(Debug)]
pub struct Cursor {
    entries: Entries,
    top: usize,
    pos: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            entries: Entries::new(),
            top: 0,
            pos: 1,
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.entries.path()
    }

    pub fn set_path(&mut self, path: PathBuf) -> Result<(), Error> {
        self.entries.update(path)
    }

    pub fn current(&self) -> &Entry {
        &self.entries[self.pos]
    }

    pub fn current_index(&self) -> usize {
        self.pos
    }

    pub fn current_top(&self) -> usize {
        self.top
    }

    pub fn entries(&self) -> EntryIterator {
        EntryIterator::new(&self.entries, self.top)
    }

    pub fn up(&mut self, rows: usize) {
        if self.pos > 0 {
            self.pos -= 1;
        }

        // Update scrolling view
        if self.top == self.pos + 1 {
            self.top = cmp::min(0, self.top - rows / 2);
        }
    }

    pub fn down(&mut self, rows: usize) {
        if self.pos < self.entries.len() {
            self.pos += 1;
        }

        // Update scrolling view
        let bottom = self.top + rows - 1;
        if bottom == self.pos {
            self.top = cmp::min(bottom, self.top + rows / 2);
        }
    }

    pub fn page_up(&mut self, rows: usize) {
        unimplemented!();
    }

    pub fn page_down(&mut self, rows: usize) {
        unimplemented!();
    }

    pub fn left(&mut self) {
        unimplemented!();
    }

    pub fn right(&mut self) {
        unimplemented!();
    }
}
