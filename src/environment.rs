use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::RuntimeError,
    token_type::{LiteralType, Token},
};

#[derive(Debug)]
pub struct Environment<T> {
    pub enclosing: Option<Rc<RefCell<Environment<T>>>>,
    pub values: HashMap<String, T>,
}

impl<T> Environment<T> {
    pub fn define(&mut self, name: String, value: T) -> () {
        self.values.insert(name.clone(), value);
    }
    pub fn get(&self, name: &Token) -> Result<&T, RuntimeError> {
        let map_value = self.values.get(name.lexeme.as_str());

        if let Some(literal_value) = map_value {
            return Ok(literal_value);
        } else {
            if let Some(enclosing_env) = &self.enclosing {
                return enclosing_env.clone().borrow_mut().get(name);
            } else {
                return Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", &name.lexeme),
                });
            }
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralType) -> Result<T, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            let assignment = self.values.insert(name.lexeme, value);
            match assignment {
                Some(literal) => Ok(literal),
                None => Ok(LiteralType::Nil),
            }
        } else {
            if let Some(enclosing_env) = &mut self.enclosing {
                return enclosing_env.borrow_mut().assign(name, value);
            } else {
                return Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", &name.lexeme),
                });
            }
        }
    }
}
