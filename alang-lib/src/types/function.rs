use std::fmt::Display;

use crate::{interpreter::Environment, parser::ast};

#[derive(Debug, Clone)]
pub struct FunctionVal {
    pub declaration: ast::Function,
    pub env: Environment,
}

impl FunctionVal {
    pub fn new(declaration: ast::Function, env: Environment) -> Self {
        Self { declaration, env }
    }
}

impl Display for FunctionVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = &self.declaration.name.expression_type;
        let name = match name {
            ast::ExpressionType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        write!(f, "Function: {}", name)
    }
}
