// Errors
// CommandDoesNotMatch {command_string}
// InvalidArgumentLength {expected, given}
use crate::node::NodeInfo;
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRequest(String),
    RequestParse(String),

    IndexOutOfBounds(usize, usize),

    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
    CapacityError(arrayvec::CapacityError<NodeInfo>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            AddrParse(e) => Some(e),
            CapacityError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            RequestParse(invalid_str) => write!(f, "Cannot parse request string: {}", invalid_str),
            IndexOutOfBounds(received, bounds) => write!(
                f,
                "Index out of bounds, given {}, expected smaller than {}",
                received, bounds
            ),
            Io(e) => e.fmt(f),
            AddrParse(e) => e.fmt(f),
            CapacityError(e) => e.fmt(f),
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

impl From<arrayvec::CapacityError<NodeInfo>> for Error {
    fn from(error: arrayvec::CapacityError<NodeInfo>) -> Self {
        Error::CapacityError(error)
    }
}
