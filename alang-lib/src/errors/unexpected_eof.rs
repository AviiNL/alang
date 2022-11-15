use std::fmt::Display;

#[derive(Debug)]
pub struct UnexpectedEOF {
    pub line: usize,
    pub column: usize,
}

impl UnexpectedEOF {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for UnexpectedEOF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected EOF")
    }
}

impl std::error::Error for UnexpectedEOF {}

impl Into<crate::errors::Error> for UnexpectedEOF {
    fn into(self) -> crate::errors::Error {
        let message = format!("Unexpected EOF").to_string();
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
