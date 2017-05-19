use mpv;
use serde_json;
use std::convert;
use std::error;
use std::fmt;
use std::io;

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
