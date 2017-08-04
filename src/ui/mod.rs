/*
 * ui/mod.rs
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

use self::chars::{ASCII_CHARS, BOX_CHARS};
use super::error::Error;
use super::player::Player;
use std::path::Path;
use std::io;
use std::io::Write;
use super::song::Song;
use termion::{clear, cursor, terminal_size};
use termion::screen::AlternateScreen;
use termion::raw::{IntoRawMode, RawTerminal};

mod chars;

pub struct UI {
    player: Player,
    term: AlternateScreen<RawTerminal<io::Stdout>>,
}

impl UI {
    pub fn new(player: Player) -> UI {
        let raw_stdout = io::stdout().into_raw_mode().expect("Unable to get stdout in raw mode");

        UI {
            player: player,
            term: AlternateScreen::from(raw_stdout),
        }
    }

    pub fn full_redraw(&mut self) -> Result<(), Error> {
        let (rows, cols) = terminal_size()?;

        write!(self.term, "{}{}", clear::All, cursor::Goto(1, 1))?;

        self.term.flush()?;

        Ok(())
    }

    pub fn redraw(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}
