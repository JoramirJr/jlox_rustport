use interpreter::Interpreter;
use token_type::LiteralType;

pub mod interpreter;
pub mod token_type;

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType;
    fn arity(&self) -> usize;
}
