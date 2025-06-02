use crate::{environment::Environment, stmt::Function, LoxCallable};

pub struct LoxFunction {
    declaration: Function,
}

impl LoxFunction {
    fn new(declaration: Function) -> LoxFunction {
        let function = LoxFunction { declaration };
        return function;
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        todo!()
    }
    fn call(
        &self,
        interpreter: Option<&mut crate::interpreter::Interpreter>,
        arguments: Vec<crate::token_type::LiteralType>,
    ) -> crate::token_type::LiteralType {
        let environment = Environment {  }
    }
}
