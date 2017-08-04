/*
 * player/mpv_player.rs
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
use mpv::{MpvHandler, MpvHandlerBuilder};
use player::MediaPlayer;
use std::env;
use std::path::{Path, PathBuf};

pub struct MpvPlayer {
    handle: MpvHandler,
    path: PathBuf,
}

impl MpvPlayer {
    pub fn new() -> Self {
        let mut mpv_builder = MpvHandlerBuilder::new().expect("Failed to initialize MPV builder");
        mpv_builder.set_option("vo", "null").expect(
            "Unable to disable video output",
        );
        let mpv_handler = mpv_builder.build().expect("Unable to build MPV handler");

        MpvPlayer {
            handle: mpv_handler,
            path: env::current_dir().expect("Unable to get current directory"),
        }
    }
}

impl MediaPlayer for MpvPlayer {
    // Player control
    fn set_pause(&mut self, pause: bool) {
        self.handle.set_property_async("pause", pause, 0).expect(
            "Unable to set player pause",
        );
    }

    fn get_pause(&self) -> bool {
        self.handle.get_property("pause").expect(
            "Unable to get player pause",
        )
    }

    // Navigator
    fn get_current_dir<'a>(&'a self) -> &'a Path {
        &self.path
    }

    fn set_current_dir(&mut self, path: &Path) -> Result<(), Error> {
        if path.is_dir() {
            self.path = PathBuf::from(path);
            Ok(())
        } else {
            let message = format!("Not a directory: {}",
                        path.to_str().unwrap_or("<invalid UTF-8>"));
            Err(Error::new(message, ErrorCause::NoCause()))
        }
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "replace"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn enqueue(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "append"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn clear(&mut self) {
        self.handle.command_async(&["playlist-clear"], 0).expect(
            "Unable to clear playlist",
        );
    }

    fn stop(&mut self) {
        self.handle.command_async(&["stop"], 0).expect(
            "Unable to stop player",
        );
    }

    fn next(&mut self) {
        self.handle
            .command_async(&["playlist-next", "weak"], 0)
            .expect("Unable to move next in the playlist");
    }

    fn prev(&mut self) {
        self.handle
            .command_async(&["playlist-prev", "weak"], 0)
            .expect("Unable to move next in the playlist");
    }
}
