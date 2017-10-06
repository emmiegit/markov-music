/*
 * markov.rs
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

use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use utils::roulette_wheel;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Chain<T>
where
    T: Eq + Hash + Debug,
{
    assocs: HashMap<T, HashMap<T, u32>>,
    start: HashMap<T, u32>,
}

impl<T> Chain<T>
where
    T: Eq + Hash + Debug,
{
    pub fn new() -> Self {
        Chain {
            assocs: HashMap::new(),
            start: HashMap::new(),
        }
    }

    pub fn associate(&mut self, prev: T, next: T) {
        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let count = probs.entry(next).or_insert(0);
        *count += 1;
    }

    pub fn start(&self) -> Option<&T> {
        let mut rng = thread_rng();
        roulette_wheel(&self.start, &mut rng)
    }

    pub fn next(&self, current: &T) -> Option<&T> {
        let mut rng = thread_rng();
        match self.assocs.get(current) {
            Some(probs) => roulette_wheel(&probs, &mut rng),
            None => None,
        }
    }

    pub fn possible_next(&self, current: &T) -> Option<&HashMap<T, u32>> {
        self.assocs.get(current)
    }
}
