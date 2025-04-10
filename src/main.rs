mod ast_printer;
mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token_type;
mod environment;

use lox::Lox;

pub fn main() {
    Lox::start();
}
