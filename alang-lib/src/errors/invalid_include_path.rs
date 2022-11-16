use std::fmt::Display;

use crate::types::RuntimeType;

#[derive(Debug)]
pub struct InvalidIncludePath {
    pub path: RuntimeType,
    pub line: usize,
    pub column: usize,
}

impl InvalidIncludePath {
    pub fn new(path: RuntimeType, line: usize, column: usize) -> Self {
        Self { path, line, column }
    }
}

impl Display for InvalidIncludePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid include path: {}", self.path)
    }
}

impl std::error::Error for InvalidIncludePath {}

impl Into<crate::errors::Error> for InvalidIncludePath {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid include path: {}", self.path).to_string();
        let line = self.line.clone();
        let column = self.column.clone();

        crate::errors::Error {
            source: Box::new(self),
            line: line,
            column: column,
            message,
        }
    }
}
