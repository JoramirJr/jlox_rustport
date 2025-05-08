mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token_type;

use lox::Lox;

pub fn main() {
    Lox::start();
}
