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
use handle::entry::{Entry, EntryType};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::{cmp, env, fs, mem};
use utils;

#[derive(Debug)]
pub struct Cursor {
    path: PathBuf,
    entries: Vec<Entry>,
    pos: usize,
}

impl Cursor {
    pub fn new() -> Self {
        let mut cursor = Cursor {
            path: env::current_dir().expect("Unable to get current directory"),
            entries: Vec::new(),
            pos: 0,
        };
        cursor.update().expect("Unable to populate directory list");
        cursor
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, mut path: PathBuf) -> Result<PathBuf, Error> {
        mem::swap(&mut self.path, &mut path);
        self.update()?;
        Ok(path)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    fn update(&mut self) -> Result<(), Error> {
        self.entries.clear();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();

            // Ignore hidden files
            if path.file_name().unwrap().to_string_lossy().starts_with(".") {
                continue;
            }

            let ftype = entry.file_type()?;

            let ftype = if ftype.is_file() {
                EntryType::File
            } else if ftype.is_dir() {
                EntryType::Directory
            } else {
                continue;
            };

            self.entries.push(Entry {
                path: path,
                ftype: ftype,
            });
        }
        self.entries.sort();
        Ok(())
    }

    pub fn current(&self) -> Cow<str> {
        self.entries[self.pos].path.to_string_lossy()
    }

    pub fn up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn down(&mut self) {
        self.pos = cmp::min(self.pos + 1, self.entries.len());
    }

    pub fn left(&mut self) {
        unimplemented!();
    }

    pub fn right(&mut self) {
        unimplemented!();
    }
}

