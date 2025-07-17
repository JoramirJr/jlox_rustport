use crate::environment::{BindableValue, Environment};
use crate::interpreter;
use crate::lox_function::LoxFunction;
use crate::lox_std::{Clock, NativeFunction};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::stmt::Function;
use crate::token_type::{Token, TokenType};
use interpreter::Interpreter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{fs, process, str::FromStr};
#[derive(Default)]
pub struct Lox {
    pub args: Vec<String>,
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Lox {
    pub fn start(&mut self) {
        let args_length = self.args.len();
        if args_length < 1 || args_length > 2 {
            println!("Usage: jlox [script]");
            process::exit(64);
        } else if args_length == 2 {
            Self::run_file(self);
        }
    }
    pub fn run_file(&mut self) {
        let file: String =
            fs::read_to_string(self.args[1].clone()).expect("File reading successful");

        let mut scanner = Scanner {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };

        let scanned_tokens = scanner.scan_tokens(file);

        let mut parser = Parser {
            tokens: Vec::new(),
            current: 0,
        };

        let statements: Vec<crate::stmt::StmtType> = parser.parse(scanned_tokens, self);

        if self.had_error {
            process::exit(65);
        }

        let mut interpreter = Interpreter {
            globals: Rc::new(RefCell::new(Environment {
                enclosing: None,
                values: HashMap::new(),
            })),
            environment: None,
        };
        interpreter.environment = Some(interpreter.globals.clone());
        interpreter.globals.borrow_mut().define(
            "clock".to_string(),
            BindableValue::NativeFunction(NativeFunction::Clock(Clock)),
        );

        interpreter.interpret(statements, self);

        if self.had_runtime_error {
            process::exit(70);
        }
    }
    pub fn runtime_error(&mut self, error: interpreter::RuntimeError) -> () {
        let message = format!("{}\n[line: {:?}]", error.message, error.token.line);
        eprintln!("{}", message);
        self.had_runtime_error = true;
    }
    pub fn error(&mut self, token: Token, message: &str) -> () {
        if token.ttype == TokenType::Eof {
            Self::report(
                self,
                &token.line,
                String::from_str(" at end").unwrap(),
                message,
            )
        } else {
            Self::report(
                self,
                &token.line,
                format!(" at '{}'", token.lexeme),
                message,
            )
        }
    }
    pub fn report(&mut self, line: &u32, location: String, message: &str) -> () {
        let report_message = format!("[line {}] Error {}: {}", line, location, message);
        eprintln!("{}", report_message);
        self.had_error = true;
    }
}
