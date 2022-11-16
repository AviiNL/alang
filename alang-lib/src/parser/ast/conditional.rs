use super::Expression;

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub body: Vec<Expression>,
    pub else_body: Option<Vec<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Box<Expression>,
}
