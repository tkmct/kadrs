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
    FromUtf8(std::string::FromUtf8Error),

    SerdeJson(serde_json::error::Error),
    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
    CapacityError(arrayvec::CapacityError<NodeInfo>),
    NoneError,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            AddrParse(e) => Some(e),
            CapacityError(e) => Some(e),
            SerdeJson(e) => Some(e),
            FromUtf8(e) => Some(e),
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
            FromUtf8(e) => e.fmt(f),
            Io(e) => e.fmt(f),
            AddrParse(e) => e.fmt(f),
            CapacityError(e) => e.fmt(f),
            SerdeJson(e) => e.fmt(f),
            NoneError => write!(f, "NoneError"),
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

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::NoneError
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::SerdeJson(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(error)
    }
}
