// Errors
// CommandDoesNotMatch {command_string}
// InvalidArgumentLength {expected, given}
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRequest(String),
    CommandParse(String),
    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            CommandParse(invalid_str) => write!(f, "Invalid command string: {}", invalid_str),
            Io(e) => e.fmt(f),
            AddrParse(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(error: std::net::AddrParseError) -> Self {
        Error::AddrParse(error)
    }
}
