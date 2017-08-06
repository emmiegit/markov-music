/*
 * error.rs
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

use mpv;
use serde_json;
use std::error;
use std::fmt;
use std::io;
use toml;
use ui::UiError;

#[derive(Debug)]
pub enum ErrorCause {
    Io(io::Error),
    Mpv(mpv::Error),
    SerdeJson(serde_json::Error),
    TomlDe(toml::de::Error),
    Curses(UiError),
    NoCause(),
}

#[derive(Debug)]
pub struct Error {
    message: String,
    error: ErrorCause,
}

impl Error {
    pub fn new<M>(message: M, error: ErrorCause) -> Self
        where M: Into<String>
    {
        Error {
            message: message.into(),
            error: error,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.error {
            ErrorCause::Io(ref e) => Some(e),
            ErrorCause::Mpv(ref e) => Some(e),
            ErrorCause::SerdeJson(ref e) => Some(e),
            ErrorCause::TomlDe(ref e) => Some(e),
            ErrorCause::Curses(ref e) => Some(e),
            ErrorCause::NoCause() => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

// Auto-conversion
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error {
            message: error::Error::description(&error).to_string(),
            error: ErrorCause::Io(error),
        }
    }
}

impl From<mpv::Error> for Error {
    fn from(error: mpv::Error) -> Error {
        Error {
            message: error::Error::description(&error).to_string(),
            error: ErrorCause::Mpv(error),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            message: error::Error::description(&error).to_string(),
            error: ErrorCause::SerdeJson(error),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Error {
        Error {
            message: format!("{}", &error),
            error: ErrorCause::TomlDe(error),
        }
    }
}
