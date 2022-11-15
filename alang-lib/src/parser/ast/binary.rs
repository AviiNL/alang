use crate::types::Operator;

use super::Expression;

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
