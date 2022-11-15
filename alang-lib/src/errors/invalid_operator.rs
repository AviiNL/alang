use crate::types::Operator;
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidOperator {
    pub operator: Operator,
    pub line: usize,
    pub column: usize,
}

impl InvalidOperator {
    pub fn new(operator: Operator, line: usize, column: usize) -> Self {
        Self {
            operator,
            line,
            column,
        }
    }
}

impl Display for InvalidOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid operator: {:?}", self.operator)
    }
}

impl std::error::Error for InvalidOperator {}

impl Into<crate::errors::Error> for InvalidOperator {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid operator: {}", self.operator);

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
