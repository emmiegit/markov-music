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

use error::{Error, ErrorCause};
use handle::entry::{Entry, EntryType};
use std::{cmp, env, fs};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Cursor {
    path: PathBuf,
    files: Vec<Entry>,
    pos: usize,
}

impl Cursor {
    pub fn new() -> Self {
        let mut cursor = Cursor {
            path: env::current_dir().expect("Unable to get current directory"),
            files: Vec::new(),
            pos: 0,
        };
        cursor.update().expect("Unable to populate directory list");
        cursor
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) -> Result<(), Error> {
        self.path = path;
        self.update()?;
        Ok(())
    }

    fn update(&mut self) -> Result<(), Error> {
        self.files.clear();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let ftype = entry.file_type()?;

            let ftype = if ftype.is_file() {
                EntryType::File
            } else if ftype.is_dir() {
                EntryType::Directory
            } else {
                continue;
            };

            self.files.push(Entry {
                path: entry.path(),
                ftype: ftype,
            });
        }
        Ok(())
    }

    pub fn current(&self) -> Result<&str, Error> {
        self.files[self.pos].path.to_str().ok_or_else(|| {
            Error::new("Path not valid UTF-8", ErrorCause::NoCause())
        })
    }

    pub fn up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn down(&mut self) {
        self.pos = cmp::min(self.pos + 1, self.files.len());
    }

    pub fn left(&mut self) {
        unimplemented!();
    }

    pub fn right(&mut self) {
        unimplemented!();
    }
}

