/*
 * handle/entry/entries.rs
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
use handle::entry::{Entry, EntryType, EntryIterator};
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::ops::Index;

lazy_static! {
    static ref PARENT_ENTRY: Entry = Entry {
        path: PathBuf::from(".."),
        ftype: EntryType::Directory,
    };
}

#[derive(Debug, Hash)]
pub struct Entries {
    path: PathBuf,
    entries: Vec<Entry>,
    has_parent: bool,
}

impl Entries {
    pub fn new() -> Self {
        let mut entries = Entries {
            path: env::current_dir().expect("Unable to get current directory"),
            entries: Vec::new(),
            has_parent: true,
        };
        let _ = entries._update();

        entries
    }

    fn _update(&mut self) -> Result<(), Error> {
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

    pub fn update(&mut self, path: PathBuf) -> Result<(), Error> {
        self.path = path;
        self._update()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn len(&self) -> usize {
        self.entries.len() + (self.has_parent as usize)
    }
}

impl Index<usize> for Entries {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        if self.has_parent {
            match index {
                0 => &PARENT_ENTRY,
                _ => &self.entries[index - 1],
            }
        } else {
            &self.entries[index]
        }
    }
}

impl<'a> IntoIterator for &'a Entries {
    type Item = (usize, &'a Entry);
    type IntoIter = EntryIterator<'a>;

    fn into_iter(self) -> EntryIterator<'a> {
        EntryIterator::new(self, 0)
    }
}
