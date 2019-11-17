// Errors
// CommandDoesNotMatch {command_string}
// InvalidArgumentLength {expected, given}
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    InvalidRequest,
    Io(std::io::Error),
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
            InvalidRequest => write!(f, "Invalid request"),
            Io(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
