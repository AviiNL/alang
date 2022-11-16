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

mod invalid_function_name;
pub use invalid_function_name::InvalidFunctionName;

mod invalid_include_path;
pub use invalid_include_path::InvalidIncludePath;

mod invalid_argument_count;
pub use invalid_argument_count::InvalidArgumentCount;

mod unexpected_eol;
pub use unexpected_eol::UnexpectedEOL;

mod unexpected_eof;
pub use unexpected_eof::UnexpectedEOF;

mod invalid_character_literal;
pub use invalid_character_literal::InvalidCharacterLiteral;

mod unexpected_token;
pub use unexpected_token::UnexpectedToken;

mod unexpected_expression;
pub use unexpected_expression::UnexpectedExpression;

mod unhandled_token;
pub use unhandled_token::UnhandledToken;

mod undefined_variable;
pub use undefined_variable::UndefinedVariable;

mod undefined_function;
pub use undefined_function::UndefinedFunction;

mod invalid_assignment;
pub use invalid_assignment::InvalidAssignment;

mod invalid_condition;
pub use invalid_condition::InvalidCondition;

mod io_error;
pub use io_error::IOError;

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
