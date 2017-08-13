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

use handle::{EntryType, Handle};
use pancurses::*;
use player::State;
use std::path;
use ui::{UiError, color};
use {ncurses, utils};

const PLAY: &'static str = "▶ ";
const PAUSE: &'static str = "▮▮";
const STOP: &'static str = "◼ ";

#[derive(Debug)]
pub struct Output<'a> {
    win: &'a mut Window,
    rows: i32,
    cols: i32,
}

impl<'a> Output<'a> {
    pub fn new(win: &'a mut Window) -> Self {
        let (rows, cols) = win.get_max_yx();
        Output {
            win: win,
            rows: rows,
            cols: cols,
        }
    }

    pub fn erase(&mut self) -> Result<(), UiError> {
        curses!(self.win.erase())?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), UiError> {
        curses!(self.win.clear())?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), UiError> {
        curses!(self.win.refresh())?;
        Ok(())
    }

    pub fn fill<T: ToChtype>(&mut self, ch: T) -> Result<(), UiError> {
        curses!(self.win.bkgd(ch.to_chtype()))?;
        Ok(())
    }

    pub fn draw_box(&mut self) -> Result<(), UiError> {
        curses!(self.win.border(
            ncurses::ACS_VLINE(),
            ncurses::ACS_VLINE(),
            ncurses::ACS_HLINE(),
            ncurses::ACS_HLINE(),
            ncurses::ACS_ULCORNER(),
            ncurses::ACS_URCORNER(),
            ncurses::ACS_LLCORNER(),
            ncurses::ACS_LRCORNER(),
        ))?;
        Ok(())
    }

    pub fn draw_divisions(&mut self) -> Result<(), UiError> {
        // Vertical divider
        curses!(self.win.mv(1, self.cols / 2 - 1))?;
        curses!(self.win.vline(ncurses::ACS_VLINE(), self.rows - 4))?;
        curses!(self.win.mvaddch(0, self.cols / 2 - 1, ncurses::ACS_TTEE()))?;

        // Horizontal divider
        curses!(self.win.mv(self.rows - 4, 1))?;
        curses!(self.win.hline(ncurses::ACS_HLINE(), self.cols - 2))?;
        curses!(self.win.mvaddch(self.rows - 4, 0, ncurses::ACS_LTEE()))?;
        curses!(self.win.mvaddch(
            self.rows - 4,
            self.cols / 2 - 1,
            ncurses::ACS_BTEE(),
        ))?;
        curses!(self.win.mvaddch(
            self.rows - 4,
            self.cols - 1,
            ncurses::ACS_RTEE(),
        ))?;

        Ok(())
    }

    pub fn draw_directory(&mut self, handle: &Handle) -> Result<(), UiError> {
        // Current directory
        let cwd = utils::compress_path(handle.get_current_dir());
        let attr = color::directory();
        curses!(self.win.attron(attr))?;
        curses!(self.win.mvaddstr(0, 1, &cwd))?;
        curses!(self.win.attroff(attr))?;

        // File listing
        let mut row = 1;
        for entry in handle.entries() {
            let path = match entry.path.file_name() {
                Some(p) => p,
                None => entry.path.as_os_str(),
            }.to_string_lossy();

            curses!(self.win.mvaddstr(row, 1, &path))?;
            if entry.ftype == EntryType::Directory {
                curses!(self.win.addch(path::MAIN_SEPARATOR))?;
            }

            row += 1;
            if row >= self.rows {
                break;
            }
        }
        Ok(())
    }

    pub fn draw_playing(&mut self, handle: &Handle) -> Result<(), UiError> {
        // Status
        let status = match handle.play_state() {
            State::Playing => PLAY,
            State::Paused => PAUSE,
            State::Stopped => STOP,
        };
        curses!(self.win.mvaddstr(0, 1, status))?;

        // Title
        let attr = color::title();
        curses!(self.win.addch(' '))?;

        curses!(self.win.attron(attr))?;
        curses!(self.win.addstr("No Title"))?;
        curses!(self.win.attroff(attr))?;

        curses!(self.win.addstr(" by "))?;

        curses!(self.win.attron(attr))?;
        curses!(self.win.addstr("Reol"))?;
        curses!(self.win.attroff(attr))?;

        // Volume
        let volume: String;
        let percent = if handle.is_muted() {
            volume = format!("{:3}%", handle.get_volume());
            &volume
        } else {
            " --%"
        };
        curses!(self.win.mvaddstr(0, self.cols - 5, percent))?;

        // Progress
        let progress = handle.play_percent() * (self.cols - 4) / 100;
        curses!(self.win.mvaddch(1, 1, '['))?;
        curses!(self.win.hline(ncurses::ACS_CKBOARD(), progress))?;
        curses!(self.win.mvaddch(1, self.cols - 2, ']'))?;

        Ok(())
    }
}
