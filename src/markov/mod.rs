/*
 * markov/mod.rs
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

pub use iter::InfiniteIterator;
pub use traits::Chainable;
use node::Node;
use rand::Rng;
use std::collection::HashMap;

mod node;
mod iter;
mod traits;

pub trait Chainable: Eq + Hash<Hasher> {}

pub struct Chain<T>
    where T: Chainable
{
    nodes: HashMap<T, Node<T>>,
    state: &T,
    mutation: f32,
    rng: Rng,
}

impl<T> Chain<T>
    where T: Chainable
{
    pub fn new(initial: T, mutation: f32) {
        let mut chain = Chain {
            nodes: HashMap::new(),
            state: &initial,
            mutation: mutation,
            rng: Rng::thread_rng(),
        };

        chain.nodes.insert(initial, Node::new(&chain));
    }

    pub fn iter(&self) -> InfiniteIterator {}

    pub fn gen(&mut self) -> T {}

    // TODO work on API/operations
}
