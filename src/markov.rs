/*
 * markov.rs
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

use rand::thread_rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::Hash;
use utils::roulette_wheel;

#[derive(PartialEq, Eq)]
pub struct Chain<T>
where
    T: Eq + Hash,
{
    assocs: HashMap<T, HashMap<T, i32>>,
    start: HashMap<T, i32>,
}

impl<T> Chain<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Chain {
            assocs: HashMap::new(),
            start: HashMap::new(),
        }
    }

    pub fn modify_weight(&mut self, prev: T, next: T, diff: i32) {
        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let weight = probs.entry(next).or_insert(0);
        *weight += diff;
    }

    pub fn clear<U>(&mut self, item: &U)
    where U: Borrow<T>,
          U: ?Sized,
    {
        if let Some(ref mut probs) = self.assocs.get_mut(item.borrow()) {
            probs.clear();
        }
    }

    pub fn start(&self) -> Option<&T> {
        let mut rng = thread_rng();
        roulette_wheel(&self.start, &mut rng)
    }

    pub fn next<U>(&self, current: &U) -> Option<&T>
    where U: Borrow<T>,
          U: ?Sized,
    {
        let mut rng = thread_rng();
        match self.assocs.get(current.borrow()) {
            Some(probs) => roulette_wheel(&probs, &mut rng),
            None => None,
        }
    }

    pub fn next_or_new<U>(&self, current: &U) -> Option<&T>
    where U: Borrow<T>,
          U: ?Sized,
   {
       self.next(current.borrow()).or_else(|| self.start())
   }

    pub fn possible_next<U>(&self, current: &U) -> Option<&HashMap<T, i32>>
    where U: Borrow<T>,
          U: ?Sized,
    {
        self.assocs.get(current.borrow())
    }

    #[inline]
    pub fn internals(&self) -> (&HashMap<T, HashMap<T, i32>>, &HashMap<T, i32>) {
        (&self.assocs, &self.start)
    }
}

impl<T> Debug for Chain<T>
where
    T: Eq + Hash,
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Chain")
         .field("assocs", &self.assocs)
         .field("start", &self.start)
         .finish()
    }
}
