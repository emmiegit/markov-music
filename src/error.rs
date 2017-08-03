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
use std::convert;
use std::error;
use std::fmt;
use std::io;
use toml;

#[derive(Debug)]
enum ErrorString {
    Static(&'static str),
    Dynamic(String),
}

#[derive(Debug)]
pub struct Error {
    message: ErrorString,
}

impl Error {
    pub fn new(msg: &'static str) -> Self {
        Error { message: ErrorString::Static(msg) }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.message {
            ErrorString::Static(ref msg) => msg,
            ErrorString::Dynamic(ref msg) => &msg,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.message {
            ErrorString::Static(ref msg) => msg,
            ErrorString::Dynamic(ref msg) => &msg[..],
        };
        write!(f, "Error: {}", s)
    }
}

// Auto-conversion
impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error {
            message: {
                let desc = error::Error::description(&error);
                ErrorString::Dynamic(desc.to_string())
            },
        }
    }
}

impl convert::From<mpv::Error> for Error {
    fn from(error: mpv::Error) -> Error {
        Error {
            message: {
                let desc = error::Error::description(&error);
                ErrorString::Dynamic(desc.to_string())
            },
        }
    }
}

impl convert::From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            message: {
                let desc = error::Error::description(&error);
                ErrorString::Dynamic(desc.to_string())
            },
        }
    }
}

impl convert::From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Error {
        Error {
            message: {
                let desc = error::Error::description(&error);
                ErrorString::Dynamic(desc.to_string())
            },
        }
    }
}
