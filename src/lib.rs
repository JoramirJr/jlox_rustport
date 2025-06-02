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
pub mod lox_function;

pub fn clock() -> f32 {
    let now = SystemTime::now();
    let millis = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    millis as f32
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType;
    fn arity(&self) -> usize;
}
