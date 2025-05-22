use grep::regex;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::error;
use std::io;

/// Alias for results used throughout the crate
pub type SearchResult<T> = Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Wrapper for regex crate errors
    GrepRegexErr(regex::Error),
    /// Wrapper for IO errors
    IoErr(io::Error),
}

/// Errors for this crate
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Create a new error
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<Error> for PyErr {
    /// Convert crate errors to pyerr to be returned in python functions
    fn from(error: Error) -> Self {
        match error.kind {
            ErrorKind::GrepRegexErr(err) => PyValueError::new_err(err.to_string()),
            ErrorKind::IoErr(err) => PyErr::from(err),
        }
    }
}

/// Convert a regex error into an error for this crate
impl From<regex::Error> for Error {
    fn from(other: regex::Error) -> Self {
        Self::new(ErrorKind::GrepRegexErr(other))
    }
}

/// Convert a io error into an error for this crate
impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Self::new(ErrorKind::IoErr(other))
    }
}

impl error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::GrepRegexErr(err) => {
                write!(f, "{}", err)
            }
            ErrorKind::IoErr(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
