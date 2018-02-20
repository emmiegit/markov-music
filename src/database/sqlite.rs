/*
 * database/handle.rs
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

use {Error, Result, StdResult};
use diesel::sqlite::SqliteConnection;
use super::Database;
use super::schema::*;

pub struct SqliteDatabase {
    conn: SqliteConnection,
}

impl SqliteDatabase {
    pub fn new<S: AsRef<str>>(url: S) -> Result<Self> {
        let conn = SqliteConnection::establish(url.as_ref())?;
        Ok(SqliteDatabase { conn: conn })
    }
}

impl Database for SqliteDatabase {
    type Error = Error;

    fn modify_weight(
        &mut self,
        prev: &str,
        next: &str,
        diff: i32,
    ) -> StdResult<(), Self::Error> {
    }

    fn clear(&mut self, prev: &str) -> StdResult<(), Self::Error> {
    }
}
