/*
 * ui/input.rs
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
use ui::Ui;

pub struct Input<'a> {
    ui: &'a mut Ui,
}

impl<'a> Input<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        Input {
            ui: ui,
        }
    }

    pub fn process_event(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}
