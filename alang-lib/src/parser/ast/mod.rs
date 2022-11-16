mod assignment;
mod binary;
mod conditional;
mod function;
mod grouping;
mod include;
mod program;
mod unary;

pub use assignment::Assignment;
pub use binary::Binary;
pub use conditional::{If, Return};
pub use function::{Call, Function};
pub use grouping::Grouping;
pub use include::Include;
pub use program::Program;
pub use unary::Unary;

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub line: usize,
    pub column: usize,
}

impl Expression {
    pub fn new(expression_type: ExpressionType, line: usize, column: usize) -> Self {
        Self {
            expression_type,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    Character(char),
    Boolean(bool),
    Type(String),

    Assignment(Assignment),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),

    If(If),
    Function(Function),
    Return(Return),
    Call(Call),

    Include(Include),
}
