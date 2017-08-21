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
use handle::entry::{Entry, Entries, EntryIterator};
use std::cmp;
use std::path::{Path, PathBuf};

 use std::io;use std::io::Write;

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

    fn adjust_view(&mut self, rows: usize) {
        let rows = rows - 1;

        if self.pos >= self.top {
            /* Up */
            let screens = (self.pos - self.top) / rows;
            self.top = cmp::min(self.entries.len() - 1, self.top + screens * rows);
        } else {
            /* Down */
            let screens = (self.top - self.pos) / rows;
            self.top = cmp::min(0, self.top - screens * rows);
        }
    }

    fn sanity(&self, rows: usize) {
write!(&mut io::stderr(), "top: {}, pos: {} -- ", self.top, self.pos).unwrap();
        assert!(self.top <= self.pos);
        assert!(self.top < self.entries.len());
        assert!(self.pos < self.entries.len());
        assert!(self.pos - self.top <= rows);
    }

    pub fn up(&mut self, off: usize, rows: usize) {
        self.pos -= cmp::min(self.pos, off);
        self.adjust_view(rows);
        self.sanity(rows);
    }

    pub fn down(&mut self, off: usize, rows: usize) {
        self.pos = cmp::min(self.entries.len() - 1, self.pos + off);
        self.adjust_view(rows);
        self.sanity(rows);
    }

    pub fn jump_top(&mut self) {
        self.top = 0;
        self.pos = 0;
    }

    pub fn jump_bottom(&mut self, rows: usize) {
        self.pos = self.entries.len() - 1;
        self.top = self.pos - rows + 1;
        self.adjust_view(rows);
        self.sanity(rows);
    }

    pub fn left(&mut self) {
        unimplemented!();
    }

    pub fn right(&mut self) {
        unimplemented!();
    }
}
