use super::Expression;

#[derive(Debug)]
pub struct If {
    pub condition: Box<Expression>,
    pub body: Vec<Expression>,
    pub else_body: Option<Vec<Expression>>,
}
