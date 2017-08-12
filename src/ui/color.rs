/*
 * ui/color.rs
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
use pancurses::*;

fn check(num: i32) {
    if num < 0 {
        panic!("Failed to initialize color pair");
    }
}

const DEFAULT_PAIR: u8 = 0;
const DIRECTORY_PAIR: u8 = 1;
static mut colors: bool = false;

#[inline(always)]
fn colors_enabled() -> bool {
    unsafe { colors }
}

pub fn init(enabled: bool) -> Result<(), Error> {
    if enabled && has_colors() {
        curses!(start_color())?;
        curses!(use_default_colors())?;

        check(init_pair(DEFAULT_PAIR as i16, -1, -1));
        check(init_pair(DIRECTORY_PAIR as i16, COLOR_BLUE, -1));
        unsafe { colors = true; }
    } else {
        unsafe { colors = false; }
    }
    Ok(())
}

pub fn default() -> chtype {
    if colors_enabled() {
        let mut attr = Attributes::new();
        attr.set_normal();
        attr.set_color_pair(ColorPair(DEFAULT_PAIR));
        chtype::from(attr)
    } else {
        0
    }
}

pub fn directory() -> chtype {
    if colors_enabled() {
        let mut attr = Attributes::new();
        attr.set_bold(true);
        attr.set_color_pair(ColorPair(DIRECTORY_PAIR));
        chtype::from(attr)
    } else {
        0
    }
}
