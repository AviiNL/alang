use super::Expression;

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expression>,
}
