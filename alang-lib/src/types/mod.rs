use crate::token::{Token, TokenType};

pub mod boolean;
pub mod character;
pub mod function;
pub mod number;
pub mod string;
// mod array;
// mod object;

#[derive(Debug, Clone)]
pub struct RuntimeType {
    pub value: RuntimeValue,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Type(String),
    String(string::StringVal),
    Number(number::NumberVal),
    Boolean(boolean::BooleanVal),
    Character(character::CharacterVal),
    Function(function::FunctionVal),
    Return(Box<RuntimeType>),
    // Array(array::ArrayVal),
    // Object(object::ObjectVal),
    Null,
}

impl BinaryOperation for RuntimeValue {
    fn operation(
        &self,
        other: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        match operator {
            Operator::Relational(Relational::Is) => {
                let other = match other {
                    RuntimeValue::Type(other) => other,
                    RuntimeValue::Number(_) => "number",
                    RuntimeValue::String(_) => "string",
                    RuntimeValue::Boolean(_) => "boolean",
                    RuntimeValue::Character(_) => "character",
                    _ => return Err(BinaryOperationError::InvalidOperationType),
                };

                match self {
                    RuntimeValue::Function(_) => {
                        return Ok(RuntimeValue::Boolean(boolean::BooleanVal {
                            value: other == "function",
                        }))
                    }
                    RuntimeValue::Number(_) => {
                        return Ok(RuntimeValue::Boolean(boolean::BooleanVal {
                            value: other == "number",
                        }))
                    }
                    RuntimeValue::String(_) => {
                        return Ok(RuntimeValue::Boolean(boolean::BooleanVal {
                            value: other == "string",
                        }))
                    }
                    RuntimeValue::Boolean(_) => {
                        return Ok(RuntimeValue::Boolean(boolean::BooleanVal {
                            value: other == "boolean",
                        }))
                    }
                    RuntimeValue::Character(_) => {
                        return Ok(RuntimeValue::Boolean(boolean::BooleanVal {
                            value: other == "character",
                        }))
                    }
                    _ => return Ok(RuntimeValue::Boolean(boolean::BooleanVal { value: false })),
                };
            }
            _ => {
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
    }
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.value {
            RuntimeValue::Type(value) => write!(f, "Type: {}", value),
            RuntimeValue::String(val) => val.fmt(f),
            RuntimeValue::Number(val) => val.fmt(f),
            RuntimeValue::Boolean(val) => val.fmt(f),
            RuntimeValue::Character(val) => val.fmt(f),
            RuntimeValue::Return(val) => val.fmt(f),
            RuntimeValue::Function(val) => val.fmt(f),

            // RuntimeValue::Array(val) => val.fmt(f),
            // RuntimeValue::Object(val) => val.fmt(f),
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

    Is, // is
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

            Operator::Relational(Relational::Is) => write!(f, "is"),

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
            TokenType::Is => Operator::Relational(Relational::Is),
            TokenType::And => Operator::Logical(Logical::And),
            TokenType::Or => Operator::Logical(Logical::Or),
            TokenType::Bang => Operator::Logical(Logical::Not),

            _ => panic!("Invalid token type for operator"),
        }
    }
}
