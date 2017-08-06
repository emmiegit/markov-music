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
use std::io::Write;
use super::Error;
use ui::chars::{ASCII_CHARS, BOX_CHARS, Chars};

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
                false => &BOX_CHARS,
            },
            rows: rows,
            cols: cols,
        }
    }

    pub fn clear(&mut self) {
        curses!(self.win.clear());
    }

    pub fn flush(&mut self) {
        curses!(self.win.refresh());
    }

    pub fn draw_box(&mut self) {
        curses!(self.win.mvaddch(1, 1, self.chars.corner_top_left));
        curses!(self.win.mvaddch(self.cols - 1, 1, self.chars.corner_top_right));

        /*
        // Draw top
        write!(self.out, "{}{}", cursor::Goto(1, 1), self.chars.corner_top_left)?;
        for _ in 0..(self.cols - 2) {
            write!(self.out, "{}", self.chars.bar_horizontal)?;
        }
        write!(self.out, "{}", self.chars.corner_top_right)?;

        // Draw bottom
        write!(
            self.out,
            "{}{}",
            cursor::Goto(1, self.rows),
            self.chars.corner_bottom_left
        )?;
        for _ in 0..(self.cols - 2) {
            write!(self.out, "{}", self.chars.bar_horizontal)?;
        }
        write!(self.out, "{}", self.chars.corner_bottom_right)?;

        // Draw sides
        write!(self.out, "{}", cursor::Goto(1, 2))?;
        for i in 0..(self.rows - 1) {
            write!(
                self.out,
                "{}{}{}{}",
                self.chars.bar_vertical,
                cursor::Right(self.cols - 2),
                self.chars.bar_vertical,
                cursor::Goto(1, i + 2)
            )?;
        }
        */
    }

    pub fn draw_directory(&mut self) {
        unimplemented!();
    }
}
