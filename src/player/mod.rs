/*
 * player/mod.rs
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

pub use self::mpv_player::MpvPlayer;
pub use self::player::Player;
use error::Error;

mod mpv_player;
mod player;

pub enum Seek {
    Absolute(f32),
    Relative(f32),
}

pub trait MediaPlayer {
    // Player control
    fn get_pause(&self) -> bool;
    fn set_pause(&mut self, pause: bool);
    fn toggle_pause(&mut self) {
        let pause = self.get_pause();
        self.set_pause(!pause);
    }

    fn get_mute(&self) -> bool;
    fn set_mute(&mut self, mute: bool);
    fn toggle_mute(&mut self) {
        let mute = self.get_mute();
        self.set_mute(!mute);
    }

    fn get_volume(&self) -> i32;
    fn set_volume(&mut self, volume: i32);
    fn change_volume(&mut self, offset: i32) {
        let volume = self.get_volume();
        self.set_volume(volume + offset);
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error>;
    fn enqueue(&mut self, song: &str) -> Result<(), Error>;
    fn clear(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn next(&mut self) -> Result<(), Error>;
    fn prev(&mut self) -> Result<(), Error>;
    fn seek(&mut self, seek: Seek) -> Result<(), Error>;
}
