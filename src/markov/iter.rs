/*
 * markov/iter.rs
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

pub struct InfiniteIterator {}

impl<T> Iterator for InfiniteIterator<T>
    where T: Chainable
{
    fn next(&mut self) -> Option<Self::Item> {}

    fn size_hint(&self) -> (usize, Option<usize>) {
        // <usize>::max_value()
        (usize::MAX, None)
    }
}
