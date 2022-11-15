use std::fmt::Display;

use crate::types::RuntimeType;

#[derive(Debug)]
pub struct InvalidCondition {
    pub condition: RuntimeType,
    pub line: usize,
    pub column: usize,
}

impl InvalidCondition {
    pub fn new(condition: RuntimeType, line: usize, column: usize) -> Self {
        Self {
            condition,
            line,
            column,
        }
    }
}

impl Display for InvalidCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid condition: {:?}", self.condition)
    }
}

impl std::error::Error for InvalidCondition {}

impl Into<crate::errors::Error> for InvalidCondition {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid condition: {:?}", self.condition).to_string();
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
