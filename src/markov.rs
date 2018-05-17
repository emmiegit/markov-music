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

use choose::{random_key, roulette_wheel};
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::Hash;
use utils::sigmoid;

#[derive(PartialEq)]
pub struct Chain<T>
where
    T: Eq + Hash,
{
    assocs: HashMap<T, HashMap<T, f32>>,
}

impl<T> Chain<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Chain {
            assocs: HashMap::new(),
        }
    }

    pub fn modify_weight(&mut self, prev: T, next: T, diff: f32) {
        assert!(diff.is_finite());

        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let weight = probs.entry(next).or_insert(0.0);
        *weight = sigmoid(*weight + diff);
    }

    pub fn clear<U>(&mut self, item: &U)
    where U: Borrow<T>,
          U: ?Sized,
    {
        if let Some(ref mut probs) = self.assocs.get_mut(item.borrow()) {
            probs.clear();
        }
    }

    pub fn start(&self, rng: &mut Rng) -> Option<&T> {
        random_key(&self.assocs, rng)
    }

    pub fn next<U>(&self, current: &U, rng: &mut Rng) -> Option<&T>
    where U: Borrow<T>,
          U: ?Sized,
    {
        match self.assocs.get(current.borrow()) {
            Some(probs) => roulette_wheel(probs, rng),
            None => None,
        }
    }

    pub fn possible_next<U>(&self, current: &U) -> Option<&HashMap<T, f32>>
    where U: Borrow<T>,
          U: ?Sized,
    {
        self.assocs.get(current.borrow())
    }
}

impl<T> Clone for Chain<T>
where
    T: Eq + Hash,
    T: Clone,
{
    fn clone(&self) -> Self {
        Chain {
            assocs: self.assocs.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.assocs.clone_from(&source.assocs);
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
         .finish()
    }
}
