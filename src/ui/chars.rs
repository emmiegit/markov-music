/*
 * ui/chars.rs
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

#[derive(Debug)]
pub struct Chars {
    pub corner_top_left: char,
    pub corner_top_right: char,
    pub corner_bottom_left: char,
    pub corner_bottom_right: char,
    pub bar_horizontal: char,
    pub bar_vertical: char,
    pub tee_north: char,
    pub tee_south: char,
    pub tee_west: char,
    pub tee_east: char,
    pub intersection: char,
}

pub static BOX_CHARS: Chars = Chars {
    corner_top_left: '┌',
    corner_top_right: '┐',
    corner_bottom_left: '└',
    corner_bottom_right: '┘',
    bar_horizontal: '─',
    bar_vertical: '│',
    tee_north: '┬',
    tee_south: '┴',
    tee_west: '├',
    tee_east: '┤',
    intersection: '┼',
};

pub static ASCII_CHARS: Chars = Chars {
    corner_top_left: '/',
    corner_top_right: '\\',
    corner_bottom_left: '\\',
    corner_bottom_right: '/',
    bar_horizontal: '-',
    bar_vertical: '|',
    tee_north: '+',
    tee_south: '+',
    tee_west: '+',
    tee_east: '+',
    intersection: '+',
};
