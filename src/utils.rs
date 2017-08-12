/*
 * utils.rs
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

use std::env;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref HOME_DIR_PATH: PathBuf = env::home_dir().expect("Unable to get home directory");
    pub static ref HOME_DIR: String = HOME_DIR_PATH
        .as_path()
        .to_string_lossy()
        .into_owned();
}

pub fn compress_path<P: AsRef<Path>>(path: P) -> String {
    let mut pathbuf;
    let path = path.as_ref();
    match path.strip_prefix(HOME_DIR_PATH.as_path()) {
        Ok(p) => {
            pathbuf = PathBuf::from("~");
            pathbuf.push(p);
            &pathbuf
        }
        Err(_) => path,
    }.to_string_lossy().into_owned()
}
