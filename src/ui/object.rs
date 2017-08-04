/*
 * ui/ui.rs
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

use config::Config;
use ui::output::Output;
use super::{Error, Player, Song};
use std::path::Path;
use std::io;
use termion::input::TermRead;
use termion::event::Key;
use termion::screen::AlternateScreen;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Ui {
    config: Config,
    player: Player,
    input: io::Stdin,
    output: AlternateScreen<RawTerminal<io::Stdout>>,
}

impl Ui {
    pub fn new(player: Player, config: Config) -> Self {
        let raw_stdout = io::stdout().into_raw_mode().expect("Unable to get stdout in raw mode");

        Ui {
            player: player,
            config: config,
            input: io::stdin(),
            output: AlternateScreen::from(raw_stdout),
        }
    }

    fn get_output(&mut self) -> Output<AlternateScreen<RawTerminal<io::Stdout>>> {
        Output::new(&mut self.output, &self.config)
    }

    pub fn full_redraw(&mut self) -> Result<(), Error> {
        let mut output = self.get_output();
        output.clear()?;
        output.draw_box()?;
        output.flush()?;

        Ok(())
    }

    pub fn redraw(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn _loop(&mut self) {
        let input = &mut self.input;
        for c in input.keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                _ => (),
            }
        }
    }
}
