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

use error::Error;
use mpv::{MpvHandler, MpvHandlerBuilder};
use self::Seek::{Absolute, Relative};

pub enum Seek {
    Absolute(f32),
    Relative(f32),
}

pub struct Player {
    handle: MpvHandler,
}

impl Player {
    pub fn new() -> Self {
        let mut mpv_builder = MpvHandlerBuilder::new().expect("Failed to initialize MPV builder");
        mpv_builder.set_option("vo", "null").expect(
            "Unable to disable video output",
        );
        let mpv_handler = mpv_builder.build().expect("Unable to build MPV handler");

        Player { handle: mpv_handler }
    }

    // Player control
    pub fn get_pause(&self) -> bool {
        self.handle.get_property("pause").expect(
            "Unable to get player pause",
        )
    }

    pub fn set_pause(&mut self, pause: bool) {
        self.handle.set_property_async("pause", pause, 0).expect(
            "Unable to set player pause",
        );
    }

    pub fn get_mute(&self) -> bool {
        self.handle.get_property("ao-mute").expect(
            "Unable to get player mute",
        )
    }

    pub fn set_mute(&mut self, mute: bool) {
        self.handle.set_property_async("ao-mute", mute, 0).expect(
            "Unable to set player mute",
        );
    }

    pub fn get_volume(&self) -> i32 {
        self.handle.get_property::<i64>("ao-volume").expect(
            "Unable to get player volume",
        ) as i32
    }

    pub fn set_volume(&mut self, volume: i32) {
        assert!(volume >= 0);
        assert!(volume <= 100);
        self.handle
            .set_property_async("ao-volume", volume as i64, 0)
            .expect("Unable to set player volume");
    }

    // Playlist
    pub fn play(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "replace"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn enqueue(&mut self, song: &str) -> Result<(), Error> {
        match self.handle.command(&["loadfile", song, "append"]) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.handle.command_async(&["playlist-clear"], 0)?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.handle.command_async(&["stop"], 0)?;

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), Error> {
        self.handle.command_async(&["playlist-next", "weak"], 0)?;

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), Error> {
        self.handle.command_async(&["playlist-prev", "weak"], 0)?;

        Ok(())
    }

    pub fn seek(&mut self, seek: Seek) -> Result<(), Error> {
        let (secs, mode) = match seek {
            Absolute(secs) => (secs, "absolute"),
            Relative(secs) => (secs, "relative"),
        };
        self.handle.command_async(
            &["seek", &format!("{}", secs), mode],
            0,
        )?;

        Ok(())
    }
}
