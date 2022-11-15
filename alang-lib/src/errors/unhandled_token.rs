use std::fmt::Display;

use crate::token::TokenType;

#[derive(Debug)]
pub struct UnhandledToken {
    pub token: TokenType,
    pub line: usize,
    pub column: usize,
}

impl UnhandledToken {
    pub fn new(token: TokenType, line: usize, column: usize) -> Self {
        Self {
            token,
            line,
            column,
        }
    }
}

impl Display for UnhandledToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unhandled Token: '{:?}'", self.token)
    }
}

impl std::error::Error for UnhandledToken {}

impl Into<crate::errors::Error> for UnhandledToken {
    fn into(self) -> crate::errors::Error {
        let message = format!("Unhandled Token: '{:?}'", self.token).to_string();
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
