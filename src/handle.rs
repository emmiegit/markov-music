/*
 * handle.rs
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

use {markov, Result};
use config::Config;
use player::Player;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
pub struct Handle {
    config: Config,
    chain: markov::Chain<String>,
    player: Player,
}

impl Handle {
    pub fn new(config: Config) -> Result<Self> {
        let player = Player::new(&config)?;

        Ok(Handle {
            config: config,
            chain: markov::Chain::new(),
            player: player,
        })
    }

    pub fn wait(&mut self) -> Result<()> {
        self.player.update()?;

        // TODO
        unimplemented!()
    }
}

impl Borrow<Config> for Handle {
    fn borrow(&self) -> &Config {
        &self.config
    }
}

impl Borrow<markov::Chain<String>> for Handle {
    fn borrow(&self) -> &markov::Chain<String> {
        &self.chain
    }
}

impl BorrowMut<markov::Chain<String>> for Handle {
    fn borrow_mut(&mut self) -> &mut markov::Chain<String> {
        &mut self.chain
    }
}

impl Borrow<Player> for Handle {
    fn borrow(&self) -> &Player {
        &self.player
    }
}

impl BorrowMut<Player> for Handle {
    fn borrow_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
