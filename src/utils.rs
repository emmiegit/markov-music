/*
 * utils.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017-2018 Ammon Smith
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

use rand::Rng;
use std::collections::HashMap;
use std::{env, str};
use std::hash::Hash;
use std::path::PathBuf;

lazy_static! {
    pub static ref HOME_DIR: PathBuf = env::home_dir().expect("Unable to get home directory");
}

pub fn roulette_wheel<'a, T: Eq + Hash>(map: &'a HashMap<T, i32>, rng: &mut Rng) -> Option<&'a T> {
    let sum = map.values().sum::<i32>() as f32;
    let mut rand = rng.next_f32();
    for (key, val) in map.iter() {
        let prob = (*val as f32) / sum;
        if rand < prob {
            return Some(&key);
        }
        rand -= prob;
    }

    None
}

#[inline]
pub fn empty_mut_str() -> &'static mut str {
    unsafe { str::from_utf8_unchecked_mut(&mut []) }
}
