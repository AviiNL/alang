use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidAssignment {
    pub line: usize,
    pub column: usize,
}

impl InvalidAssignment {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for InvalidAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Assignment")
    }
}

impl std::error::Error for InvalidAssignment {}

impl Into<crate::errors::Error> for InvalidAssignment {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid Assignment").to_string();
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
