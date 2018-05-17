/*
 * choose.rs
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
use std::hash::Hash;

macro_rules! len {
    ($map:tt) => { $map.len() as f32 }
}

pub fn random_key<'a, K: Eq + Hash, V>(map: &'a HashMap<K, V>, rng: &mut Rng) -> Option<&'a K> {
    let index = rng.next_f32() * len!(map);
    map.keys().nth(index as usize)
}

pub fn roulette_wheel<'a, K: Eq + Hash>(map: &'a HashMap<K, f32>, rng: &mut Rng) -> Option<&'a K> {
    let mut rand = rng.next_f32() * len!(map);
    for (key, &weight) in map.iter() {
        if rand < weight {
            return Some(key);
        }

        rand -= weight;
    }

    None
}
