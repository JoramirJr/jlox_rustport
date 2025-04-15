use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token_type::{LiteralType, Token},
};

type VarValue = Option<LiteralType>;

pub struct Environment {
    pub values: HashMap<String, VarValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Option<LiteralType>) -> () {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<VarValue, RuntimeError> {
        let map_value = self.values.get(name.lexeme.as_str()).unwrap();

        if let &None = map_value {
            return Err(RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", &name.lexeme),
            });
        } else {
            return Ok(map_value.clone());
        }
    }
}
