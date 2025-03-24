mod lox;
mod expr;
mod ast_printer;
mod parser;
mod scanner;
mod token_type;

use std::env;
use lox::Lox;

pub fn main() {
    let lox = Lox {
        args: env::args().collect(),
        had_error: false,
    };
    lox.start();
}

// impl Config {
//     pub fn update_value(new_value: i32) {
//         let mut config = CONFIG.lock().unwrap();
//         config.value = new_value; // ✅ Mutate global state
//     }

//     pub fn get_value() -> i32 {
//         CONFIG.lock().unwrap().value
//     }
// }

// pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config { value: 42 }));
