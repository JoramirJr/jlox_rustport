use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token_type::{LiteralType, Token},
};

type VarValue = Option<LiteralType>;

struct Environment {
    values: HashMap<String, VarValue>,
}

impl Environment {
    fn define(&mut self, name: String, value: Option<LiteralType>) -> () {
        self.values.insert(name, value);
    }
    fn get(&self, name: &Token) -> Result<VarValue, RuntimeError> {
        let map_value = self.values.get(name.lexeme.as_str()).unwrap();

        if let &None = map_value {
            return Err(RuntimeError {
                token: *name,
                message: &format!("Undefied variable '{}'.", *name.lexeme),
            });
        } else {
            return Ok(*map_value);
        }
    }
}
