mod ast_printer;
mod expr;
mod lox;
mod parser;
mod scanner;
mod token_type;

use lox::Lox;

pub fn main() {
    Lox::start();
}
