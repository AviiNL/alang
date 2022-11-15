use crate::types::{Operator, RuntimeType};
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidOperationType {
    pub left_type: RuntimeType,
    pub right_type: Option<RuntimeType>,
    pub operator: Operator,
}

impl InvalidOperationType {
    pub fn new(
        left_type: RuntimeType,
        right_type: Option<RuntimeType>,
        operator: Operator,
    ) -> Self {
        Self {
            left_type,
            right_type,
            operator,
        }
    }
}

impl Display for InvalidOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let right = &self.right_type;
        let operator = &self.operator;

        if right.is_some() {
            let right = right.clone().unwrap();

            write!(
                f,
                "Invalid operation, mismatching types: {:?} {} {:?}",
                self.left_type.value, operator, right.value
            )
        } else {
            write!(
                f,
                "Invalid operation: {}{:?}",
                self.operator, self.left_type.value
            )
        }
    }
}

impl std::error::Error for InvalidOperationType {}

impl Into<crate::errors::Error> for InvalidOperationType {
    fn into(self) -> crate::errors::Error {
        let operator = &self.operator;
        let right = &self.right_type;

        let message = if right.is_some() {
            let right = right.clone().unwrap();
            format!(
                "Invalid operation, mismatching types: {:?} {} {:?}",
                self.left_type.value, operator, right.value
            )
        } else {
            format!("Invalid operation: {}{:?}", operator, self.left_type.value)
        };
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
