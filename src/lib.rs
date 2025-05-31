use std::time::{SystemTime, UNIX_EPOCH};

use interpreter::Interpreter;
use token_type::LiteralType;

pub mod interpreter;
pub mod token_type;

pub trait Callable {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType;
    fn arity(&self) -> usize;
}
pub struct LoxCallable;

impl LoxCallable {
    fn to_string() -> String {}
}

impl Callable for LoxCallable {
    fn arity(&self) -> usize {
        return 0;
    }
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType {
        let now = SystemTime::now();
        let millis = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let f32_millis: f32 = millis.try_into();

        return LiteralType::F32();
    }
}
