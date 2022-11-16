use crate::types::Operator;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Operator,
    pub right: Box<Expression>,
}
