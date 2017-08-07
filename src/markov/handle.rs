/*
 * markov/handle.rs
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

use super::Chain;
use error::{Error, ErrorCause};
use player::{MediaPlayer, Player, Seek};
use std::{cmp, env};
use std::path::{Path, PathBuf};

struct Cursor {
    pub path: PathBuf,
    files: Vec<PathBuf>,
    pos: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            path: env::current_dir().expect("Unable to get current directory"),
            files: Vec::new(),
            pos: 0,
        }
    }

    fn current(&self) -> Result<&str, Error> {
        self.files[self.pos]
            .to_str()
            .ok_or_else(|| Error::new("Path not valid UTF-8", ErrorCause::NoCause()))
    }

    fn up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn down(&mut self) {
        self.pos = cmp::min(self.pos + 1, self.files.len());
    }

    fn parent(&mut self) {
        unimplemented!();
    }
}

pub struct Handle {
    chain: Chain,
    player: Player,
    cursor: Cursor,
}

impl Handle {
    pub fn new(chain: Chain, player: Player) -> Self {
        Handle {
            chain: chain,
            player: player,
            cursor: Cursor::new(),
        }
    }

    // Navigator
    pub fn get_current_dir(&self) -> &Path {
        &self.cursor.path
    }

    pub fn set_current_dir(&mut self, path: &Path) -> Result<(), Error> {
        if path.is_dir() {
            self.cursor.path = PathBuf::from(path);
            Ok(())
        } else {
            let message = format!("Not a directory: {}",
                        path.to_str().unwrap_or("<invalid UTF-8>"));
            Err(Error::new(message, ErrorCause::NoCause()))
        }
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
    }

    pub fn cursor_down(&mut self) {
        self.cursor.down();
    }

    pub fn cursor_parent(&mut self) {
        self.cursor.parent();
    }

    // Player
    pub fn get_pause(&self) -> bool {
        self.player.get_pause()
    }

    pub fn toggle_pause(&mut self) {
        self.player.toggle_pause();
    }

    pub fn play(&mut self) -> Result<(), Error> {
        let song = self.cursor.current()?;
        self.player.play(song)?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.player.stop();

        Ok(())
    }

    pub fn toggle_mute(&mut self) {
        self.player.toggle_mute();
    }

    pub fn get_volume(&self) -> i32 {
        self.player.get_volume()
    }

    pub fn change_volume(&mut self, offset: i32) {
        self.player.change_volume(offset);
    }

    pub fn seek_begin(&mut self) -> Result<(), Error> {
        self.player.seek(Seek::Absolute(0.0))?;

        Ok(())
    }

    pub fn seek_end(&mut self) -> Result<(), Error> {
        self.player.seek(Seek::Absolute(-0.001))?;

        Ok(())
    }

    pub fn seek(&mut self, secs: f32) -> Result<(), Error> {
        assert!(secs.is_finite());
        self.player.seek(Seek::Relative(secs))?;

        Ok(())
    }

    // Markov chain
    pub fn add(&mut self) -> Result<(), Error> {
        let song = self.cursor.current()?;
        unimplemented!();
    }

    pub fn shuffle(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn next(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn prev(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn repeat(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn loop_back(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn like(&mut self) {
        unimplemented!();
    }

    pub fn dislike(&mut self) {
        unimplemented!();
    }

    pub fn tired(&mut self) {
        unimplemented!();
    }

    pub fn random(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}
