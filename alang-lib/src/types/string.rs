use super::{
    boolean::BooleanVal, Arithmatic, BinaryOperation, BinaryOperationError, Operator, Relational,
    RuntimeValue,
};
use crate::token::{Token, TokenType};

use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct StringVal {
    pub(crate) value: String,
}

impl Debug for StringVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Display for StringVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BinaryOperation for StringVal {
    fn operation(
        &self,
        other_raw: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        let other = match &other_raw {
            RuntimeValue::String(s) => s,
            _ => return Err(BinaryOperationError::InvalidOperationType),
        };

        match operator {
            Operator::Arithmatic(Arithmatic::Plus) => {
                let mut new_value = self.value.clone();
                new_value.push_str(&other.value);
                Ok(RuntimeValue::String(StringVal { value: new_value }))
            }
            // Equal
            Operator::Relational(Relational::Equal) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value == other.value,
            })),
            // Not Equal
            Operator::Relational(Relational::NotEqual) => Ok(RuntimeValue::Boolean(BooleanVal {
                value: self.value != other.value,
            })),
            _ => Err(BinaryOperationError::InvalidOperator),
        }
    }
}

impl From<String> for StringVal {
    fn from(s: String) -> Self {
        StringVal { value: s }
    }
}

impl From<Token> for StringVal {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::String(value) => StringVal { value },
            _ => unreachable!(),
        }
    }
}
