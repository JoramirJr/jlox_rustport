use std::time::{SystemTime, UNIX_EPOCH};

use interpreter::Interpreter;
use token_type::LiteralType;

use crate::environment::BindableValue;

pub mod ast_printer;
pub mod environment;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod lox_function;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token_type;

pub fn clock() -> f32 {
    let now = SystemTime::now();
    let millis = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    millis as f32
}

pub trait LoxCallable {
    fn call(&self, interpreter: Option<&mut Interpreter>, arguments: Vec<BindableValue>);
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
