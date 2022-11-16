use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidArgumentCount {
    pub name: String,
    pub received: usize,
    pub expected: usize,
    pub line: usize,
    pub column: usize,
}

impl InvalidArgumentCount {
    pub fn new(name: &str, received: usize, expected: usize, line: usize, column: usize) -> Self {
        Self {
            name: name.to_string(),
            received,
            expected,
            line,
            column,
        }
    }
}

impl Display for InvalidArgumentCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid argument count for function '{}', got {} expected {}",
            self.name, self.received, self.expected
        )
    }
}

impl std::error::Error for InvalidArgumentCount {}

impl Into<crate::errors::Error> for InvalidArgumentCount {
    fn into(self) -> crate::errors::Error {
        let message = format!(
            "Invalid argument count for function '{}', got {} expected {}",
            self.name, self.received, self.expected
        )
        .to_string();
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
