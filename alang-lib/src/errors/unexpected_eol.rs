use std::fmt::Display;

#[derive(Debug)]
pub struct UnexpectedEOL {
    pub line: usize,
    pub column: usize,
}

impl UnexpectedEOL {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for UnexpectedEOL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected EOL")
    }
}

impl std::error::Error for UnexpectedEOL {}

impl Into<crate::errors::Error> for UnexpectedEOL {
    fn into(self) -> crate::errors::Error {
        let message = format!("Unexpected EOL").to_string();
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
