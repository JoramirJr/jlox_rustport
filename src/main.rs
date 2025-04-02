mod ast_printer;
mod expr;
mod lox;
mod parser;
mod scanner;
mod token_type;
mod interpreter;

use lox::Lox;

pub fn main() {
    Lox::start();
}
