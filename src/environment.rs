use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token_type::{LiteralType, Token},
};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, LiteralType>,
}

type DefaultResult = Result<LiteralType, RuntimeError>;

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralType) -> () {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> DefaultResult {
        let map_value = self.values.get(name.lexeme.as_str());

        if let Some(literal_value) = map_value {
            return Ok(literal_value.clone());
        } else {
            if let Some(enclosing_env) = &self.enclosing {
                return enclosing_env.get(name);
            } else {
                return Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", &name.lexeme),
                });
            }
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralType) -> DefaultResult {
        if self.values.contains_key(&name.lexeme) {
            let assignment = self.values.insert(name.lexeme, value);
            match assignment {
                Some(literal) => Ok(literal),
                None => Ok(LiteralType::Nil),
            }
        } else {
            if let Some(enclosing_env) = &mut self.enclosing {
                return enclosing_env.assign(name, value);
            } else {
                return Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", &name.lexeme),
                });
            }
        }
    }
}
