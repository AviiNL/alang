use std::fmt::Display;

#[derive(Debug)]
pub struct UnhandledCharacter {
    pub character: char,
    pub line: usize,
    pub column: usize,
}

impl UnhandledCharacter {
    pub fn new(character: char, line: usize, column: usize) -> Self {
        Self {
            character,
            line,
            column,
        }
    }
}

impl Display for UnhandledCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unhandled character: '{:?}'", self.character)
    }
}

impl std::error::Error for UnhandledCharacter {}

impl Into<crate::errors::Error> for UnhandledCharacter {
    fn into(self) -> crate::errors::Error {
        let message = format!("Unhandled character: '{}'", self.character).to_string();
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
