use super::{
    boolean::BooleanVal, string::StringVal, Arithmatic, BinaryOperation, BinaryOperationError,
    Operator, Relational, RuntimeValue,
};
use crate::token::{Token, TokenType};

use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct CharacterVal {
    pub(crate) value: char,
}

impl Debug for CharacterVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Display for CharacterVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BinaryOperation for CharacterVal {
    fn operation(
        &self,
        other_raw: &RuntimeValue,
        operator: Operator,
    ) -> Result<RuntimeValue, BinaryOperationError> {
        let other = match &other_raw {
            RuntimeValue::Character(s) => s,
            _ => return Err(BinaryOperationError::InvalidOperationType),
        };

        match operator {
            Operator::Arithmatic(Arithmatic::Plus) => {
                let mut new_value = String::from(self.value);
                new_value.push_str(&other.value.to_string());
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

impl From<char> for CharacterVal {
    fn from(c: char) -> Self {
        Self { value: c }
    }
}

impl From<Token> for CharacterVal {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::Character(value) => CharacterVal { value },
            _ => unreachable!(),
        }
    }
}
