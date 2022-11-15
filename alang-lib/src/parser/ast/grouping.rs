use super::Expression;

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Expression>,
}
