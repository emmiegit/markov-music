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

pub use mpv_player::MpvPlayer;
pub use player::Player;
use error::Error;

mod mpv_player;
mod player;

pub trait MediaPlayer {
    // Player control
    fn set_pause(&mut self, pause: bool);
    fn get_pause(&self) -> bool;
    fn toggle_pause(&mut self) {
        let pause = self.get_pause();
        self.set_pause(!pause);
    }

    // Playlist
    fn play(&mut self, song: &str) -> Result<(), Error>;
    fn enqueue(&mut self, song: &str) -> Result<(), Error>;
    fn clear(&mut self);
    fn stop(&mut self);
    fn next(&mut self);
    fn prev(&mut self);
}
