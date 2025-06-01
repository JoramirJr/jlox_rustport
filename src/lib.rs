use std::time::{SystemTime, UNIX_EPOCH};

use interpreter::Interpreter;
use token_type::LiteralType;

pub mod ast_printer;
pub mod environment;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token_type;

pub trait Callable {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType;
    fn arity(&self) -> usize;
}

#[derive(Debug)]
pub struct LoxCallable;

impl LoxCallable {
    fn to_string() -> String {
        todo!()
    }
}

impl Callable for LoxCallable {
    fn arity(&self) -> usize {
        return 0;
    }
    fn call(
        &self,
        _interpreter: Option<&mut Interpreter>,
        _arguments: Vec<LiteralType>,
    ) -> LiteralType {
        let now = SystemTime::now();
        let millis = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let f32_millis = millis as f32;

        return LiteralType::F32(f32_millis);
    }
}
