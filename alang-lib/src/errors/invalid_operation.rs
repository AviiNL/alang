use crate::types::{Operator, RuntimeType};
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidOperation {
    pub left_type: RuntimeType,
    pub right_type: RuntimeType,
    pub operator: Operator,
}

impl InvalidOperation {
    pub fn new(left_type: RuntimeType, right_type: RuntimeType, operator: Operator) -> Self {
        Self {
            left_type,
            right_type,
            operator,
        }
    }
}

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid operation type: {:?}", self.left_type)
    }
}

impl std::error::Error for InvalidOperation {}

impl Into<crate::errors::Error> for InvalidOperation {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid operation type: {}", self.left_type);
        let line = self.left_type.line;
        let column = self.left_type.column;

        crate::errors::Error {
            source: Box::new(self),
            line,
            column,
            message,
        }
    }
}
