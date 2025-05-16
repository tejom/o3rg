use grep::regex;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::error;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    GrepRegexErr(regex::Error),
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        match error.kind {
            ErrorKind::GrepRegexErr(err) => PyValueError::new_err(err.to_string()),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(other: regex::Error) -> Self {
        Self::new(ErrorKind::GrepRegexErr(other))
    }
}

impl error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::GrepRegexErr(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
