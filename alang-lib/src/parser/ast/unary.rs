use crate::types::Operator;

use super::Expression;

#[derive(Debug)]
pub struct Unary {
    pub operator: Operator,
    pub right: Box<Expression>,
}
