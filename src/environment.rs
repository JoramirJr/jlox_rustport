use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    lox_function::LoxFunction,
    lox_std::NativeFunction,
    token_type::{LiteralType, Token},
};

#[derive(Debug, Clone)]
pub enum BindableValue {
    Literal(LiteralType),
    Function(LoxFunction),
    NativeFunction(NativeFunction),
}

pub struct Environment<'a> {
    pub enclosing: Option<&'a mut Environment<'a>>,
    pub values: HashMap<String, BindableValue>,
}

impl Environment<'_> {
    pub fn define(&mut self, name: String, value: BindableValue) -> () {
        self.values.insert(name.clone(), value);
    }
    pub fn get(&self, name: &Token) -> Result<BindableValue, RuntimeError> {
        let map_value = self.values.get(name.lexeme.as_str());

        if let Some(value) = map_value {
            return Ok(value.clone());
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
    pub fn assign(
        &mut self,
        name: Token,
        value: BindableValue,
    ) -> Result<BindableValue, RuntimeError> {
        let value_clone = value.clone();

        if self.values.contains_key(&name.lexeme) {
            let assignment = self.values.insert(name.lexeme, value);
            match assignment {
                Some(literal) => Ok(literal),
                None => Ok(value_clone),
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
