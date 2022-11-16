use std::fmt::Display;

use crate::parser::ast::ExpressionType;

#[derive(Debug)]
pub struct UnexpectedExpression {
    pub provided: ExpressionType,
    pub expected: Option<ExpressionType>,
    pub line: usize,
    pub column: usize,
}

impl UnexpectedExpression {
    pub fn new(
        provided: ExpressionType,
        expected: Option<ExpressionType>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            provided,
            expected,
            line,
            column,
        }
    }
}

impl Display for UnexpectedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expected.is_some() {
            write!(
                f,
                "Unexpected Expression: {:?} expected {:?}",
                self.provided, self.expected
            )
        } else {
            write!(f, "Unexpected Expression: {:?}", self.provided)
        }
    }
}

impl std::error::Error for UnexpectedExpression {}

impl Into<crate::errors::Error> for UnexpectedExpression {
    fn into(self) -> crate::errors::Error {
        let expected = &self.expected;

        let message = if expected.is_some() {
            let expected = expected.as_ref().unwrap();
            format!(
                "Unexpected Expression {:?} expected {:?}",
                self.provided, expected
            )
            .to_string()
        } else {
            format!("Unexpected Expression {:?}", self.provided).to_string()
        };
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
