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
use ui::input::Input;
use ui::output::Output;
use pancurses::*;
use super::{Error, Player};
use std::io;
use std::sync::Mutex;

pub struct Ui {
    win: Window,
    config: Config,
    player: Player,
}

impl Ui {
    pub fn new(player: Player, config: Config) -> Self {
        let win = initscr();
        curses!(cbreak());
        curses!(nl());
        curses!(noecho());
        curses!(win.nodelay(true));

        Ui {
            win: win,
            player: player,
            config: config,
        }
    }

    pub fn full_redraw(&mut self) {
        let mut output = Output::new(&mut self.win, &self.config);
        output.clear();
        output.draw_box();
        output.flush();
    }

    pub fn redraw(&mut self) {
        unimplemented!();
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        curses!(endwin());
    }
}
