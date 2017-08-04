/*
 * player/player.rs
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
use player::MediaPlayer;
use player::mpv_player::MpvPlayer;
use std::path::Path;

pub enum Player {
    Mpv(MpvPlayer),
}

impl Player {
    fn get_inst(&self) -> &MediaPlayer {
        match self {
            &Player::Mpv(ref x) => x,
        }
    }

    fn get_mut_inst(&mut self) -> &mut MediaPlayer {
        match self {
            &mut Player::Mpv(ref mut x) => x,
        }
    }
}

impl MediaPlayer for Player {
    fn set_pause(&mut self, pause: bool) {
        self.get_mut_inst().set_pause(pause);
    }

    fn get_pause(&self) -> bool {
        self.get_inst().get_pause()
    }

    fn play(&mut self, song: &str) -> Result<(), Error> {
        self.get_mut_inst().play(song)
    }

    fn get_current_dir<'a>(&'a self) -> &'a Path {
        self.get_inst().get_current_dir()
    }

    fn set_current_dir(&mut self, path: &Path) -> Result<(), Error> {
        self.get_mut_inst().set_current_dir(path)
    }

    fn enqueue(&mut self, song: &str) -> Result<(), Error> {
        self.get_mut_inst().enqueue(song)
    }

    fn clear(&mut self) {
        self.get_mut_inst().clear();
    }

    fn stop(&mut self) {
        self.get_mut_inst().stop();
    }

    fn next(&mut self) {
        self.get_mut_inst().next();
    }

    fn prev(&mut self) {
        self.get_mut_inst().prev();
    }
}
