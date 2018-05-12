/*
 * error.rs
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

use mpd::error as mpd;
use self::Error::*;
use std::{fmt, num, io};
use std::str::Utf8Error;
use toml;

pub use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    StaticMsg(&'static str),
    Msg(String),
    Io(io::Error),
    IntParse(num::ParseIntError),
    Utf8(Utf8Error),
    TomlDe(toml::de::Error),
    MpdParse(mpd::ParseError),
    MpdProto(mpd::ProtoError),
    MpdServer(mpd::ServerError),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            StaticMsg(s) => s,
            Msg(ref s) => s,
            Io(ref e) => e.description(),
            IntParse(ref e) => e.description(),
            Utf8(ref e) => e.description(),
            TomlDe(ref e) => e.description(),
            MpdParse(ref e) => e.description(),
            MpdProto(ref e) => e.description(),
            MpdServer(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            StaticMsg(_) | Msg(_) => None,
            Io(ref e) => Some(e),
            IntParse(ref e) => Some(e),
            Utf8(ref e) => Some(e),
            TomlDe(ref e) => Some(e),
            MpdParse(ref e) => Some(e),
            MpdProto(ref e) => Some(e),
            MpdServer(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", StdError::description(self))
    }
}

// Auto-conversion
impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Msg(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::IntParse(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::TomlDe(error)
    }
}

impl From<mpd::Error> for Error {
    fn from(error: mpd::Error) -> Self {
        use self::mpd::Error::*;

        match error {
            Io(e) => Error::Io(e),
            Parse(e) => Error::MpdParse(e),
            Proto(e) => Error::MpdProto(e),
            Server(e) => Error::MpdServer(e),
        }
    }
}
