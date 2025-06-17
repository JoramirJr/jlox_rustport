use std::env;

use jlox_rustport::lox::Lox;

pub fn main() {
    let mut lox = Lox {
        args: env::args().collect(),
        had_error: false,
        had_runtime_error: false,
    };
    lox.start();
}
