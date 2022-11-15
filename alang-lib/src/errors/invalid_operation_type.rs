use crate::types::{Operator, RuntimeType};
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidOperationType {
    pub left_type: RuntimeType,
    pub right_type: RuntimeType,
    pub operator: Operator,
}

impl InvalidOperationType {
    pub fn new(left_type: RuntimeType, right_type: RuntimeType, operator: Operator) -> Self {
        Self {
            left_type,
            right_type,
            operator,
        }
    }
}

impl Display for InvalidOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid operation, mismatching types: {:?} {} {:?}",
            self.left_type.value, self.operator, self.right_type.value
        )
    }
}

impl std::error::Error for InvalidOperationType {}

impl Into<crate::errors::Error> for InvalidOperationType {
    fn into(self) -> crate::errors::Error {
        let message = format!(
            "Invalid operation, mismatching types: {:?} {} {:?}",
            self.left_type.value, self.operator, self.right_type.value
        );
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
