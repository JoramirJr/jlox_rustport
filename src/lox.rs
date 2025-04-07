use crate::expr::ExpressionType;
use crate::interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token_type::{Token, TokenType};
use std::{fs, process, str::FromStr};

use interpreter::Interpreter;
use std::env;
use std::sync::{LazyLock, Mutex, MutexGuard};
#[derive(Default)]
pub struct Lox {
    pub args: Vec<String>,
    pub had_error: bool,
    pub had_runtime_error: bool,
}

pub static LOX_SINGLETON: LazyLock<Mutex<Lox>> = LazyLock::new(|| {
    Mutex::new(Lox {
        args: env::args().collect(),
        had_error: false,
        had_runtime_error: false,
    })
});

impl Lox {
    pub fn start() {
        let lox_singleton = LOX_SINGLETON.lock();

        match lox_singleton {
            Ok(lox) => {
                let args_length = lox.args.len();
                if args_length < 1 || args_length > 2 {
                    println!("Usage: jlox [script]");
                    process::exit(64);
                } else if args_length == 2 {
                    Self::run_file(lox);
                }
            }
            Err(err) => {
                panic!("Lox singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn run_file(lox: MutexGuard<'_, Lox>) {
        let file: String = fs::read_to_string(&lox.args[1]).expect("File reading successful");
        std::mem::drop(lox);

        let scanned_tokens = Scanner::scan_tokens(file);
        let expr: Option<ExpressionType> = Parser::parse(scanned_tokens);

        match expr {
            Some(expr) => {
                Interpreter::interpret(expr);
            }
            None => {
                return;
            }
        }
    }
    pub fn runtime_error(error: interpreter::RuntimeError) -> () {
        let lox_singleton = LOX_SINGLETON.lock();
        match lox_singleton {
            Ok(mut singleton) => {
                let message = format!("{}\n[line: {:?}]", error.message, error.token.line);
                eprintln!("{}", message);
                singleton.had_runtime_error = true;
                std::mem::drop(singleton);
            }
            Err(err) => {
                panic!("Singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn error(token: Token, message: &str) -> () {
        if token.ttype == TokenType::Eof {
            Self::report(&token.line, String::from_str(" at end").unwrap(), message)
        } else {
            Self::report(&token.line, format!(" at '{}'", token.lexeme), message)
        }
    }
    pub fn report(line: &u32, location: String, message: &str) -> () {
        let lox_singleton = LOX_SINGLETON.lock();
        match lox_singleton {
            Ok(mut singleton) => {
                let report_message = format!("[line {}] Error {}: {}", line, location, message);
                eprintln!("{}", report_message);
                singleton.had_error = true;
                std::mem::drop(singleton);
            }
            Err(err) => {
                panic!("Singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
}
