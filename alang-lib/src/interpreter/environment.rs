use std::collections::HashMap;

use crate::types::RuntimeValue;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    pub(crate) variables: HashMap<String, RuntimeValue>,
    constants: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        // Default global environment
        let variables = HashMap::new();

        Environment {
            parent: None,
            variables,
            constants: Vec::new(),
        }
    }

    pub fn new_with_parent(parent: Environment) -> Self {
        Environment {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn with_default_scope(mut self) -> Self {
        self.variables.insert(
            "PI".to_string(),
            RuntimeValue::Number(std::f64::consts::PI.into()),
        );
        self
    }

    pub fn set(&mut self, symbol: &str, value: RuntimeValue, is_const: bool) -> RuntimeValue {
        if self.variables.contains_key(symbol) {
            self.assign(symbol, value.clone());
            return value;
        }

        self.variables.insert(symbol.to_string(), value.clone());

        if is_const {
            self.constants.push(symbol.to_string());
        }

        value
    }

    pub fn assign(&mut self, symbol: &str, value: RuntimeValue) -> RuntimeValue {
        let env = self.resolve(symbol);

        match env {
            Some(e) => {
                let symbol = symbol.to_owned();

                if e.constants.contains(&symbol) {
                    panic!("Cannot assign to constant {}", symbol);
                }

                e.variables.insert(symbol.clone(), value.clone());

                value
            }
            None => panic!("Variable {} not defined", symbol),
        }
    }

    pub fn resolve(&mut self, symbol: &str) -> Option<&mut Environment> {
        if self.variables.contains_key(symbol) {
            return Some(self);
        }

        match &mut self.parent {
            Some(p) => p.resolve(symbol),
            None => None,
        }
    }

    pub fn get(&mut self, symbol: &str) -> Option<RuntimeValue> {
        let env = self.resolve(symbol);

        match env {
            Some(e) => e.variables.get(symbol).map(|v| v.clone()),
            None => None,
        }
    }
}
