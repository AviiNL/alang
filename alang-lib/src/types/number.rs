use super::{
    boolean::BooleanVal, Arithmatic, BinaryOperation, BinaryOperationError, Operator, Relational,
    RuntimeValue,
};
use crate::token::{Token, TokenType};

use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct NumberVal {
    pub(crate) value: f64,
}

impl Debug for NumberVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl Display for NumberVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl BinaryOperation for NumberVal {
    fn operation(
        &self,
        other_raw: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        let other = match &other_raw {
            RuntimeValue::Number(s) => s,
            _ => return Err(BinaryOperationError::InvalidOperationType),
        };

        match operator {
            Operator::Arithmatic(Arithmatic::Plus) => Ok(RuntimeValue::Number(NumberVal {
                value: self.value.clone() + other.value,
            })),
            Operator::Arithmatic(Arithmatic::Minus) => Ok(RuntimeValue::Number(NumberVal {
                value: self.value.clone() - other.value,
            })),
            Operator::Arithmatic(Arithmatic::Multiply) => Ok(RuntimeValue::Number(NumberVal {
                value: self.value.clone() * other.value,
            })),
            Operator::Arithmatic(Arithmatic::Divide) => Ok(RuntimeValue::Number(NumberVal {
                value: self.value.clone() / other.value,
            })),
            Operator::Arithmatic(Arithmatic::Modulo) => Ok(RuntimeValue::Number(NumberVal {
                value: self.value.clone() % other.value,
            })),
            Operator::Arithmatic(Arithmatic::Exponentiation) => {
                Ok(RuntimeValue::Number(NumberVal {
                    value: self.value.clone().powf(other.value),
                }))
            }

            Operator::Relational(Relational::Equal) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value == other.value,
            })),

            Operator::Relational(Relational::NotEqual) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value != other.value,
            })),

            Operator::Relational(Relational::LessThan) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value < other.value,
            })),

            Operator::Relational(Relational::LessThanOrEqual) => {
                Ok(RuntimeValue::Boolean(BooleanVal {
                    value: self.value <= other.value,
                }))
            }

            Operator::Relational(Relational::GreaterThan) => {
                Ok(RuntimeValue::Boolean(BooleanVal {
                    value: self.value > other.value,
                }))
            }

            Operator::Relational(Relational::GreaterThanOrEqual) => {
                Ok(RuntimeValue::Boolean(BooleanVal {
                    value: self.value >= other.value,
                }))
            }

            _ => Err(BinaryOperationError::InvalidOperator),
        }
    }
}

impl From<f64> for NumberVal {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Token> for NumberVal {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::Number(value) => NumberVal { value },
            _ => unreachable!(),
        }
    }
}
