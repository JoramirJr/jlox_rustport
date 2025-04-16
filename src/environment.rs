use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token_type::{LiteralType, Token},
};

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
        let enclosing_env_option = self.enclosing;
        let map_value = self.values.get(name.lexeme.as_str()).unwrap();

        if let Some(enclosing_env) = enclosing_env_option {
            return enclosing_env.get(name);
        }

        if let &LiteralType::Nil = map_value {
            return Err(RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", &name.lexeme),
            });
        } else {
            return Ok(map_value.clone());
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
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", &name.lexeme),
                token: name,
            })
        }
    }
}
