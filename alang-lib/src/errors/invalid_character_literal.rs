use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidCharacterLiteral {
    pub line: usize,
    pub column: usize,
}

impl InvalidCharacterLiteral {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for InvalidCharacterLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid character literal")
    }
}

impl std::error::Error for InvalidCharacterLiteral {}

impl Into<crate::errors::Error> for InvalidCharacterLiteral {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid character literal").to_string();
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
