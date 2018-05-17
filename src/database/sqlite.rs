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

use {diesel, Error, Result, StdResult};
use diesel::sqlite::SqliteConnection;
use super::Database;
use super::models::*;
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
        song: &str,
        next: &str,
        diff: i32,
    ) -> Result<()> {
        self.conn.transaction::<(), Error, _>(|| {
            use self::associations::dsl;

            let row = associations::table
                .find((song, next))
                .first::<Association>(&self.conn)
                .optional()?;

            let weight = row.map(|assoc| assoc.weight).unwrap_or(0) + diff;
            let new_assoc = NewAssociation {
                song: song,
                next: next,
                weight: weight,
            };

            diesel::replace_into(starters::table)
                .values(&[new_assoc])
                .execute(&self.conn)?;

            Ok(())
        })
    }

    fn clear(&mut self, song: &str) -> Result<()> {
        self.conn.transaction::<(), Error, _>(|| {
            use self::associations::dsl;

            diesel::delete(associations::table.filter(dsl::song.eq(song)))
                .execute(&self.conn)?;

            Ok(())
        })
    }
}
