mod invalid_operator;
pub use invalid_operator::InvalidOperator;

mod invalid_operation;
pub use invalid_operation::InvalidOperation;

mod invalid_operation_type;
pub use invalid_operation_type::InvalidOperationType;

mod unhandled_character;
pub use unhandled_character::UnhandledCharacter;

mod invalid_escape_character;
pub use invalid_escape_character::InvalidEscapeCharacter;

mod unexpected_eol;
pub use unexpected_eol::UnexpectedEOL;

mod unexpected_eof;
pub use unexpected_eof::UnexpectedEOF;

mod invalid_character_literal;
pub use invalid_character_literal::InvalidCharacterLiteral;

mod unexpected_token;
pub use unexpected_token::UnexpectedToken;

mod unhandled_token;
pub use unhandled_token::UnhandledToken;

mod undefined_variable;
pub use undefined_variable::UndefinedVariable;

mod invalid_assignment;
pub use invalid_assignment::InvalidAssignment;

pub struct Error {
    source: Box<dyn std::error::Error>,
    line: usize,
    column: usize,
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}
