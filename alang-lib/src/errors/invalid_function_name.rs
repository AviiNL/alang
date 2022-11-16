use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidFunctionName {
    pub line: usize,
    pub column: usize,
}

impl InvalidFunctionName {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for InvalidFunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid function name")
    }
}

impl std::error::Error for InvalidFunctionName {}

impl Into<crate::errors::Error> for InvalidFunctionName {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid function name").to_string();
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
