use std::fmt::Display;

#[derive(Debug)]
pub struct UndefinedVariable {
    pub identifier: String,
    pub line: usize,
    pub column: usize,
}

impl UndefinedVariable {
    pub fn new(identifier: String, line: usize, column: usize) -> Self {
        Self {
            identifier,
            line,
            column,
        }
    }
}

impl Display for UndefinedVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Undefined Variable {:?}", self.identifier)
    }
}

impl std::error::Error for UndefinedVariable {}

impl Into<crate::errors::Error> for UndefinedVariable {
    fn into(self) -> crate::errors::Error {
        let message = format!("Undefined Variable {:?}", self.identifier).to_string();
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
