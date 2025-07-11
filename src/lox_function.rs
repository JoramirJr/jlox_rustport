use std::collections::HashMap;

use crate::{
    environment::{BindableValue, Environment},
    interpreter::Interpreter,
    stmt::Function,
    token_type::LiteralType,
    LoxCallable,
};

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub declaration: Function,
}

impl LoxFunction {
    fn new(declaration: Function) -> LoxFunction {
        let function = LoxFunction { declaration };
        return function;
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    fn call(&self, interpreter: Option<&mut Interpreter>, arguments: Vec<BindableValue>) {
        let interpreter = interpreter.unwrap();
        let mut environment = Environment {
            enclosing: interpreter.globals.clone(),
            values: HashMap::new(),
        };

        for (idx, _) in self.declaration.params.iter().enumerate() {
            environment.define(
                self.declaration.params.get(idx).unwrap().lexeme.clone(),
                arguments.get(idx).unwrap().clone(),
            );
        }

        let _ = interpreter.execute_block(self.declaration.body.clone());
    }
    fn to_string(&self) -> String {
        format!("<fn {} >", self.declaration.name.lexeme)
    }
}
