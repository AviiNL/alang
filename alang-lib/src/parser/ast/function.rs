use super::Expression;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Box<Expression>,
    pub parameters: Vec<Expression>,
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: Box<Expression>,
    pub parameters: Vec<Expression>,
}
