use crate::types::Operator;
use std::fmt::Display;

#[derive(Debug)]
pub struct IOError {
    pub io_error: std::io::Error,
    pub line: usize,
    pub column: usize,
}

impl IOError {
    pub fn new(io_error: std::io::Error, line: usize, column: usize) -> Self {
        Self {
            io_error,
            line,
            column,
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO Error: {}", self.io_error)
    }
}

impl std::error::Error for IOError {}

impl Into<crate::errors::Error> for IOError {
    fn into(self) -> crate::errors::Error {
        let message = format!("IO Error: {}", self.io_error);

        let line = self.line;
        let column = self.column;

        crate::errors::Error {
            source: Box::new(self),
            line: line,
            column: column,
            message,
        }
    }
}
