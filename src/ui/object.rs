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
use pancurses::*;
use std::ptr;
use super::Error;
use ui::output::Output;

pub struct Ui<'a> {
    win: Window,
    config: &'a Config,
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Result<Self, Error> {
        let win = initscr();
        let _ = mousemask(ALL_MOUSE_EVENTS, ptr::null_mut());
        set_title("Markov Music Player");
        curses!(cbreak())?;
        curses!(nl())?;
        curses!(noecho())?;
        curses!(win.keypad(true))?;
        curses!(win.nodelay(true))?;

        Ok(Ui {
            win: win,
            config: config,
        })
    }

    pub fn get_window(&self) -> &Window {
        &self.win
    }

    pub fn get_window_mut(&mut self) -> &mut Window {
        &mut self.win
    }

    pub fn full_redraw(&mut self) -> Result<(), Error> {
        let mut output = Output::new(&mut self.win, &self.config);
        output.clear()?;
        output.draw_box()?;
        output.flush()?;

        Ok(())
    }

    pub fn redraw(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}

impl<'a> Drop for Ui<'a> {
    fn drop(&mut self) {
        curses!(endwin()).expect("Calling endwin() failed");
    }
}
