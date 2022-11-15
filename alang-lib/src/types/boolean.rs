use super::{BinaryOperation, BinaryOperationError, Operator, Relational, RuntimeValue};
use crate::token::{Token, TokenType};

use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct BooleanVal {
    pub(crate) value: bool,
}

impl Debug for BooleanVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl Display for BooleanVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl BinaryOperation for BooleanVal {
    fn operation(
        &self,
        other_raw: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        let other = match &other_raw {
            RuntimeValue::Boolean(s) => s.value,
            _ => {
                return Err(BinaryOperationError::InvalidOperationType);
            }
        };

        match operator {
            Operator::Relational(Relational::Equal) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value == other,
            })),
            Operator::Relational(Relational::NotEqual) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value != other,
            })),
            _ => Err(BinaryOperationError::InvalidOperator),
        }
    }
}

impl From<bool> for BooleanVal {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<Token> for BooleanVal {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::Boolean(value) => BooleanVal { value },
            _ => unreachable!(),
        }
    }
}
