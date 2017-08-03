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

use error::Error;
use rand::{thread_rng, Rng};
use serde_json;
use song::Song;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::path::Path;

fn roulette_wheel<'a, T: Eq + Hash>(map: &'a HashMap<T, u32>, rng: &mut Rng) -> Option<&'a T> {
    let sum = map.values().sum::<u32>() as f32;
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

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MarkovChain {
    assocs: HashMap<Song, HashMap<Song, u32>>,
    start: HashMap<Song, u32>,
}

impl MarkovChain {
    pub fn new() -> Self {
        MarkovChain {
            assocs: HashMap::new(),
            start: HashMap::new(),
        }
    }

    pub fn read(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;

        Ok(config)
    }

    pub fn associate(&mut self, prev: Song, next: Song) {
        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let count = probs.entry(next).or_insert(0);
        *count += 1;
    }

    pub fn start(&self) -> Option<&Song> {
        let mut rng = thread_rng();
        roulette_wheel(&self.start, &mut rng)
    }

    pub fn next(&self, current: &Song) -> Option<&Song> {
        let mut rng = thread_rng();
        match self.assocs.get(current) {
            Some(probs) => roulette_wheel(&probs, &mut rng),
            None => None,
        }
    }

    pub fn possible_next(&self, current: &Song) -> Option<&HashMap<Song, u32>> {
        self.assocs.get(current)
    }

    pub fn write(&self, path: &Path) -> Result<(), Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        serde_json::to_writer(file, self)?;

        Ok(())
    }
}
