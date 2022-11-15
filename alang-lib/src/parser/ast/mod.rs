mod assignment;
mod binary;
mod grouping;
mod program;
mod unary;

pub use assignment::Assignment;
pub use binary::Binary;
pub use grouping::Grouping;
pub use program::Program;
pub use unary::Unary;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ExpressionType {
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    Character(char),
    Boolean(bool),

    Assignment(Assignment),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
}
