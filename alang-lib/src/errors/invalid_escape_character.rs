use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidEscapeCharacter {
    pub character: char,
    pub line: usize,
    pub column: usize,
}

impl InvalidEscapeCharacter {
    pub fn new(character: char, line: usize, column: usize) -> Self {
        Self {
            character,
            line,
            column,
        }
    }
}

impl Display for InvalidEscapeCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid escape character: '{:?}'", self.character)
    }
}

impl std::error::Error for InvalidEscapeCharacter {}

impl Into<crate::errors::Error> for InvalidEscapeCharacter {
    fn into(self) -> crate::errors::Error {
        let message = format!("Invalid escape character: '{}'", self.character).to_string();
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
