use std::fmt::Display;

use crate::token::TokenType;

#[derive(Debug)]
pub struct UnexpectedToken {
    pub provided: TokenType,
    pub expected: Option<TokenType>,
    pub line: usize,
    pub column: usize,
}

impl UnexpectedToken {
    pub fn new(
        provided: TokenType,
        expected: Option<TokenType>,
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

impl Display for UnexpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expected.is_some() {
            write!(
                f,
                "Unexpected Token: {:?} expected {:?}",
                self.provided, self.expected
            )
        } else {
            write!(f, "Unexpected Token: {:?}", self.provided)
        }
    }
}

impl std::error::Error for UnexpectedToken {}

impl Into<crate::errors::Error> for UnexpectedToken {
    fn into(self) -> crate::errors::Error {
        let message = if self.expected.is_some() {
            format!(
                "Unexpected Token {:?} expected {:?}",
                self.provided, self.expected
            )
            .to_string()
        } else {
            format!("Unexpected Token {:?}", self.provided).to_string()
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
