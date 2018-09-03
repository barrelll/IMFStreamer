use std::convert::From;
use std::io;
use std::str;

#[derive(Debug)]
pub enum AtomError {
    UnreadableError(str::Utf8Error),
    ParseIntError(io::Error),
    ParseAtomNameErr(io::Error),
    EOFError,
}

impl From<str::Utf8Error> for AtomError {
    fn from(error: str::Utf8Error) -> Self {
        AtomError::UnreadableError(error)
    }
}

impl From<io::Error> for AtomError {
    fn from(error: io::Error) -> Self {
        AtomError::ParseIntError(error)
    }
}