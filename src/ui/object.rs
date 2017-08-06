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
use super::{Error, Player};
use ui::output::Output;

pub struct Ui {
    win: Window,
    config: Config,
    player: Player,
}

impl Ui {
    pub fn new(player: Player, config: Config) -> Result<Self, Error> {
        let win = initscr();
    println!("cbreak");
        curses!(cbreak())?;
    println!("nl");
        curses!(nl())?;
    println!("noecho");
        curses!(noecho())?;
        /*
    println!("keypad");
        curses!(win.keypad(true))?;
        */
        /*
    println!("nodelay");
        curses!(win.nodelay(true))?;
        */

        Ok(Ui {
            win: win,
            player: player,
            config: config,
        })
    }

    pub fn get_window<'a>(&'a self) -> &'a Window {
        &self.win
    }

    pub fn get_mut_window<'a>(&'a mut self) -> &'a mut Window {
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

impl Drop for Ui {
    fn drop(&mut self) {
        curses!(endwin()).expect("Calling endwin() failed");
    }
}
