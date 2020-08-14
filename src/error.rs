use crate::parser::Status;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Protocol(Status),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Protocol(e) => write!(f, "protocol: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Status> for Error {
    fn from(s: Status) -> Self {
        Error::Protocol(s)
    }
}
