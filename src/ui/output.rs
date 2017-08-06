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

use config::Config;
use pancurses::*;
use ui::chars::{ASCII_CHARS, BOX_CHARS, Chars};
use ui::UiError;

pub struct Output<'a> {
    win: &'a mut Window,
    chars: &'static Chars,
    rows: i32,
    cols: i32,
}

impl<'a> Output<'a> {
    pub fn new(win: &'a mut Window, config: &Config) -> Self {
        let (rows, cols) = win.get_max_yx();
        Output {
            win: win,
            chars: match config.ascii_chars {
                true => &ASCII_CHARS,
                false => &ASCII_CHARS,
            },
            rows: rows,
            cols: cols,
        }
    }

    pub fn clear(&mut self) -> Result<(), UiError> {
        curses!(self.win.clear())?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), UiError> {
        curses!(self.win.refresh())?;

        Ok(())
    }

    pub fn draw_box(&mut self) -> Result<(), UiError> {
        curses!(self.win.border(
                self.chars.bar_vertical,
                self.chars.bar_vertical,
                self.chars.bar_horizontal,
                self.chars.bar_horizontal,
                self.chars.corner_top_left,
                self.chars.corner_top_right,
                self.chars.corner_bottom_left,
                self.chars.corner_bottom_right,
        ))?;

        Ok(())
    }

    pub fn draw_directory(&mut self) -> Result<(), UiError> {
        unimplemented!();
    }
}
