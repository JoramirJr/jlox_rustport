use std::time::{SystemTime, UNIX_EPOCH};

use interpreter::Interpreter;
use token_type::LiteralType;

use crate::{environment::BindableValue, expr::ExpressionType, interpreter::RuntimeError};

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

pub mod lox_std {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        environment::BindableValue,
        expr::{ExpressionType, Literal},
        interpreter::RuntimeError,
        token_type::LiteralType,
        LoxCallable,
    };

    #[derive(Debug, Clone)]
    pub enum NativeFunction {
        Clock(Clock),
    }

    #[derive(Debug, Clone)]
    pub struct Clock;

    impl LoxCallable for Clock {
        fn call(
            &self,
            _: Option<&mut crate::interpreter::Interpreter>,
            _: Vec<crate::environment::BindableValue>,
        ) -> Result<BindableValue, RuntimeError> {
            let now = SystemTime::now();
            let time_elapsed = now
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs_f32();

            return Ok(BindableValue::Literal(LiteralType::F32(time_elapsed)));
        }

        fn arity(&self) -> usize {
            return 0;
        }

        fn to_string(&self) -> String {
            return "<native fn>".to_string();
        }
    }
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<BindableValue>,
    ) -> Result<BindableValue, RuntimeError>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
