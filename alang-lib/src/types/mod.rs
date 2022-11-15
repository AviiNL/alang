use crate::token::{Token, TokenType};

pub mod boolean;
pub mod character;
pub mod number;
pub mod string;
// mod array;
// mod object;
// mod function;

#[derive(Debug, Clone)]
pub struct RuntimeType {
    pub value: RuntimeValue,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(string::StringVal),
    Number(number::NumberVal),
    Boolean(boolean::BooleanVal),
    Character(character::CharacterVal),
    // Array(array::ArrayVal),
    // Object(object::ObjectVal),
    // Function(function::FunctionVal),
    Null,
}

impl BinaryOperation for RuntimeValue {
    fn operation(
        &self,
        other: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        match self {
            RuntimeValue::String(value) => value.operation(other, operator),
            RuntimeValue::Number(value) => value.operation(other, operator),
            RuntimeValue::Boolean(value) => value.operation(other, operator),
            RuntimeValue::Character(value) => value.operation(other, operator),
            // RuntimeValue::Array(value) => value.operation(other, operator),
            // RuntimeValue::Object(value) => value.operation(other, operator),
            // RuntimeValue::Function(value) => value.operation(other, operator),
            // RuntimeValue::Null => Err(BinaryOperationError::Null),
            _ => Err(BinaryOperationError::InvalidOperation),
        }
    }
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.value {
            RuntimeValue::String(val) => val.fmt(f),
            RuntimeValue::Number(val) => val.fmt(f),
            RuntimeValue::Boolean(val) => val.fmt(f),
            RuntimeValue::Character(val) => val.fmt(f),
            // RuntimeValue::Array(val) => val.fmt(f),
            // RuntimeValue::Object(val) => val.fmt(f),
            // RuntimeValue::Function(val) => val.fmt(f),
            RuntimeValue::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperationError {
    InvalidOperationType,
    InvalidOperation,
    InvalidOperator,
}

pub trait BinaryOperation {
    fn operation(
        &self,
        other: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError>;
}

#[derive(Debug, Clone)]
pub enum Operator {
    Arithmatic(Arithmatic),
    Relational(Relational),
    Logical(Logical),
}

#[derive(Debug, Clone)]
pub enum Arithmatic {
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    Exponentiation, // ^
}

#[derive(Debug, Clone)]
pub enum Relational {
    Equal,    // ==
    NotEqual, // !=

    LessThan,    // <
    GreaterThan, // >

    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
}

#[derive(Debug, Clone)]
pub enum Logical {
    And, // &&
    Or,  // ||
    Not, // !
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Arithmatic(Arithmatic::Plus) => write!(f, "+"),
            Operator::Arithmatic(Arithmatic::Minus) => write!(f, "-"),
            Operator::Arithmatic(Arithmatic::Multiply) => write!(f, "*"),
            Operator::Arithmatic(Arithmatic::Divide) => write!(f, "/"),
            Operator::Arithmatic(Arithmatic::Modulo) => write!(f, "%"),
            Operator::Arithmatic(Arithmatic::Exponentiation) => write!(f, "^"),

            Operator::Relational(Relational::Equal) => write!(f, "=="),
            Operator::Relational(Relational::NotEqual) => write!(f, "!="),

            Operator::Relational(Relational::LessThan) => write!(f, "<"),
            Operator::Relational(Relational::GreaterThan) => write!(f, ">"),

            Operator::Relational(Relational::LessThanOrEqual) => write!(f, "<="),
            Operator::Relational(Relational::GreaterThanOrEqual) => write!(f, ">="),

            Operator::Logical(Logical::And) => write!(f, "&&"),
            Operator::Logical(Logical::Or) => write!(f, "||"),
            Operator::Logical(Logical::Not) => write!(f, "!"),
        }
    }
}

impl From<Token> for Operator {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::Plus => Operator::Arithmatic(Arithmatic::Plus),
            TokenType::Minus => Operator::Arithmatic(Arithmatic::Minus),
            TokenType::Star => Operator::Arithmatic(Arithmatic::Multiply),
            TokenType::Slash => Operator::Arithmatic(Arithmatic::Divide),
            TokenType::Percent => Operator::Arithmatic(Arithmatic::Modulo),
            TokenType::Caret => Operator::Arithmatic(Arithmatic::Exponentiation),
            TokenType::EqualEqual => Operator::Relational(Relational::Equal),
            TokenType::BangEqual => Operator::Relational(Relational::NotEqual),
            TokenType::Less => Operator::Relational(Relational::LessThan),
            TokenType::Greater => Operator::Relational(Relational::GreaterThan),
            TokenType::LessEqual => Operator::Relational(Relational::LessThanOrEqual),
            TokenType::GreaterEqual => Operator::Relational(Relational::GreaterThanOrEqual),
            TokenType::And => Operator::Logical(Logical::And),
            TokenType::Or => Operator::Logical(Logical::Or),
            TokenType::Bang => Operator::Logical(Logical::Not),

            _ => panic!("Invalid token type for operator"),
        }
    }
}
