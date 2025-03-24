mod lox;
mod expr;
mod ast_printer;
mod parser;
mod scanner;
mod token_type;

use std::env;
use lox::Lox;

pub fn main() {
    let main = Lox {
        args: env::args().collect(),
        had_error: false,
    };
    main.start();
}
