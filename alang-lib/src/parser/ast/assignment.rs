use super::Expression;

#[derive(Debug)]
pub struct Assignment {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
