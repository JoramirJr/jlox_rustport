use crate::interpreter::RuntimeError;
use crate::token_type::{Token, TokenType};
use crate::{parser, scanner};
use std::{fs, io, io::Write, process, str::FromStr};

use parser::Parser;
use scanner::Scanner;
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
            Ok(singleton) => {
                let args_length = singleton.args.len();
                if args_length < 1 || args_length > 2 {
                    println!("Usage: jlox [script]");
                    process::exit(64);
                } else if args_length == 2 {
                    Self::run_file(singleton);
                }
                //  else if args_length == 2 {
                //     Self::run_prompt(self);
                // }
            }
            Err(err) => {
                panic!("Singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn run_file(singleton: MutexGuard<'_, Lox>) {
        let file = fs::read_to_string(&singleton.args[1]).expect("File reading successful");
        let scanner: Scanner = Scanner {
            source: file,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        std::mem::drop(singleton);
        let scanned_tokens = scanner.scan_tokens();
        let mut parser = Parser::new(scanned_tokens);
        let expr = parser.parse();

        match expr {
            Some(expr) => {
                
            }
            None => {
                return;
            }
        }
    }

    pub fn run_prompt(mut self) -> io::Error {
        let mut input = String::new();
        let mut std_out_handler: io::Stdout = io::stdout();

        loop {
            let _ = std_out_handler.write_all("> ".as_bytes());
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    Self::run(&input, &mut std_out_handler);
                    self.had_error = false;
                }
                //Not sure if ctrl+D, to exit the program, generates an error response
                Err(error) => return error,
            }
        }
    }

    pub fn run(source: &String, std_out_handler: &mut io::Stdout) {
        source.split(" ").for_each(|token| {
            let _ = std_out_handler.write_all(token.as_bytes());
        });
    }
    pub fn error(&self, token: Token, message: &str, singleton: Lox) -> () {
        if token.ttype == TokenType::Eof {
            Self::report(
                self,
                &token.line,
                String::from_str(" at end").unwrap(),
                message,
                singleton,
            )
        } else {
            Self::report(
                self,
                &token.line,
                format!(" at '{}'", token.lexeme),
                message,
                singleton,
            )
        }
    }
    fn runtime_error(error: RuntimeError) -> () {
        let message = format!("{}\n[line: {:?}]", error.message, error.token.line);
        eprintln!("{}", message);
    }
    pub fn report(&self, line: &u32, location: String, message: &str, mut singleton: Lox) -> () {
        let report_message = format!("[line {}] Error {}: {}", line, location, message);
        eprintln!("{}", report_message);
        singleton.had_error = true;
        std::mem::drop(singleton);
    }
}
