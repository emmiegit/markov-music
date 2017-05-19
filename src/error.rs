use mpv;
use serde_json;
use std::convert;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub fn new(message: &'static str) -> Error {
        Error {
            message: message,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

// Auto-conversion
impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error {
            //FIXME
            //message: error.description(),
            message: "I/O error",
        }
    }
}

impl convert::From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            //FIXME
            //message: error.description(),
            message: "JSON decoding error",
        }
    }
}

impl convert::From<mpv::Error> for Error {
    fn from(error: mpv::Error) -> Error {
        Error {
            //FIXME
            //message: error.description(),
            message: "MPV error",
        }
    }
}
