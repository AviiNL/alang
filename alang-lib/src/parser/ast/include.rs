use super::Expression;

#[derive(Debug, Clone)]
pub struct Include {
    pub path: Box<Expression>,
}
