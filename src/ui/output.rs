/*
 * ui/output.rs
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

use ui::chars::{ASCII_CHARS, BOX_CHARS, Chars};
use config::Config;
use std::io::Write;
use super::Error;
use termion::{clear, cursor, terminal_size};

pub struct Output<'a, W>
    where W: 'a, W: Write
{
    out: &'a mut W,
    config: &'a Config,
}

impl<'a, W> Output<'a, W>
    where W: Write
{
    pub fn new(out: &'a mut W, config: &'a Config) -> Self {
        Output {
            out: out,
            config: config,
        }
    }

    fn get_chars(&self) -> &'static Chars {
        match self.config.ascii_chars {
            true => &ASCII_CHARS,
            false => &BOX_CHARS,
        }
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        write!(self.out, "{}", clear::All)?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.out.flush()?;

        Ok(())
    }

    pub fn draw_box(&mut self) -> Result<(), Error> {
        let (rows, cols) = terminal_size()?;
        let chars = self.get_chars();

        // Draw top
        write!(self.out, "{}{}", cursor::Goto(1, 1), chars.corner_top_left)?;
        for _ in 0..(cols-2) {
            write!(self.out, "{}", chars.bar_horizontal)?;
        }
        write!(self.out, "{}{}", chars.corner_top_right, cursor::Goto(2, 1))?;

        // Draw sides
        for _ in 0..(rows-2) {
            write!(self.out, "{}{}{}{}{}",
                   cursor::Down(1),
                   chars.bar_vertical,
                   cursor::Right(cols - 1),
                   chars.bar_vertical,
                   cursor::Left(cols - 1),
            )?;
        }

        // Draw bottom
        write!(self.out, "{}{}{}{}",
               chars.corner_bottom_left,
               cursor::Goto(rows, cols),
               chars.corner_bottom_right,
               cursor::Goto(rows, 2),
        )?;
        for _ in 0..(cols-2) {
            write!(self.out, "{}", chars.bar_horizontal)?;
        }

        Ok(())
    }
}
