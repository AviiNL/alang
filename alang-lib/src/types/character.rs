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
            RuntimeValue::Character(s) => Some(s),
            _ => None,
        };

        if other.is_some() {
            let other = other.unwrap();
            return match operator {
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
                Operator::Relational(Relational::NotEqual) => {
                    Ok(RuntimeValue::Boolean(BooleanVal {
                        value: self.value != other.value,
                    }))
                }
                _ => Err(BinaryOperationError::InvalidOperator),
            };
        }

        // math on characters with numbers
        let other = match &other_raw {
            RuntimeValue::Number(s) => Some(s),
            _ => None,
        };

        if other.is_some() {
            let other = other.unwrap();
            return match operator {
                Operator::Arithmatic(Arithmatic::Plus) => {
                    // get numeric value of character
                    let mut new_value = self.value as u8;
                    new_value += other.value as u8;

                    // back to character
                    let new_value = new_value as char;
                    Ok(RuntimeValue::Character(CharacterVal { value: new_value }))
                }
                Operator::Arithmatic(Arithmatic::Minus) => {
                    // get numeric value of character
                    let mut new_value = self.value as u8;
                    new_value -= other.value as u8;

                    // back to character
                    let new_value = new_value as char;
                    Ok(RuntimeValue::Character(CharacterVal { value: new_value }))
                }
                // Equal
                Operator::Relational(Relational::Equal) => Ok(RuntimeValue::Boolean(BooleanVal {
                    value: self.value as i32 == other.value as i32,
                })),
                // Not Equal
                Operator::Relational(Relational::NotEqual) => {
                    Ok(RuntimeValue::Boolean(BooleanVal {
                        value: self.value as i32 != other.value as i32,
                    }))
                }
                _ => Err(BinaryOperationError::InvalidOperator),
            };
        }

        Err(BinaryOperationError::InvalidOperationType)
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
