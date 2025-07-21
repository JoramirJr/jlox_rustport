use std::collections::HashMap;

use crate::{
    environment::{BindableValue, Environment},
    expr::{ExpressionType, Literal},
    interpreter::{self, Interpreter},
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
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<BindableValue>,
    ) -> ExpressionType {
        let interpreter = interpreter.unwrap();
        let mut environment = Environment {
            enclosing: Some(interpreter.globals.clone()),
            values: HashMap::new(),
        };

        for (idx, param) in self.declaration.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments.get(idx).unwrap().clone());
        }

        let _ = interpreter.execute_block(self.declaration.body.clone());

        return ExpressionType::Literal(Literal {
            value: LiteralType::Nil,
        });
    }
    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
