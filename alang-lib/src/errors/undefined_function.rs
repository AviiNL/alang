use std::fmt::Display;

#[derive(Debug)]
pub struct UndefinedFunction {
    pub identifier: String,
    pub line: usize,
    pub column: usize,
}

impl UndefinedFunction {
    pub fn new(identifier: String, line: usize, column: usize) -> Self {
        Self {
            identifier,
            line,
            column,
        }
    }
}

impl Display for UndefinedFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Undefined Function {:?}", self.identifier)
    }
}

impl std::error::Error for UndefinedFunction {}

impl Into<crate::errors::Error> for UndefinedFunction {
    fn into(self) -> crate::errors::Error {
        let message = format!("Undefined Function {:?}", self.identifier).to_string();
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
