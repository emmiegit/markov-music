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

use error::Error;
use handle::Handle;
use pancurses::*;
use std::ptr;
use ui::color;
use ui::output::Output;

#[derive(Debug)]
pub struct Ui {
    pub main: Window,
    pub files: Window,
    pub markov: Window,
    pub player: Window,
}

impl Ui {
    pub fn new(enable_color: bool) -> Result<Self, Error> {
        let win = initscr();
        let _ = mousemask(ALL_MOUSE_EVENTS, ptr::null_mut());
        set_title("Markov Music Player");
        curses!(cbreak())?;
        curses!(nl())?;
        curses!(noecho())?;
        curses!(win.keypad(true))?;
        color::init(enable_color)?;
        //curses!(win.nodelay(true))?;

        let (rows, cols) = win.get_max_yx();
        let files = curses_res!(win.subwin(rows - 5, cols / 2 - 2, 1, 1))?;
        let markov = curses_res!(win.subwin(rows - 5, cols / 2 - 1, 1, cols / 2))?;
        let player = curses_res!(win.subwin(2, cols - 2, rows - 3, 1))?;

        Ok(Ui {
            main: win,
            files: files,
            markov: markov,
            player: player,
        })
    }

    pub fn get_key(&self) -> Option<Input> {
        self.main.getch()
    }

    pub fn full_redraw(&mut self, handle: &Handle) -> Result<(), Error> {
        let mut main = Output::new(&mut self.main);
        let mut files = Output::new(&mut self.files);
        let mut markov = Output::new(&mut self.markov);
        let mut player = Output::new(&mut self.player);

        main.clear()?;
        main.draw_box()?;
        main.draw_divisions()?;
        files.draw_directory(handle)?;
        player.draw_playing(handle)?;
        main.flush()?;
        Ok(())
    }

    pub fn redraw(&mut self, handle: &Handle) -> Result<(), Error> {
        let mut main = Output::new(&mut self.main);
        let mut files = Output::new(&mut self.files);
        let mut markov = Output::new(&mut self.markov);
        let mut player = Output::new(&mut self.player);

        files.erase()?;
        files.draw_directory(handle)?;
        player.erase()?;
        player.draw_playing(handle)?;
        main.flush()?;
        Ok(())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        curses!(endwin()).expect("Calling endwin() failed");
    }
}
