use std::collections::HashMap;

use crate::{
    environment::{BindableValue, Environment},
    expr::{ExpressionType, Literal},
    interpreter::{self, Interpreter, RuntimeError},
    stmt::Function,
    token_type::LiteralType,
    LoxCallable,
};

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub declaration: Function,
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<BindableValue>,
    ) -> Result<BindableValue, RuntimeError> {
        let interpreter = interpreter.unwrap();
        let mut environment = Environment {
            enclosing: Some(interpreter.globals.clone()),
            values: HashMap::new(),
        };

        for (idx, param) in self.declaration.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments.get(idx).unwrap().clone());
        }

        return interpreter
            .execute_block(self.declaration.body.clone(), environment)?
            .unwrap();
    }
    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
