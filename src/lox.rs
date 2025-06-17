use crate::interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token_type::{Token, TokenType};
use interpreter::Interpreter;
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

        let scanned_tokens = Scanner::scan_tokens(file);

        let mut parser = Parser {
            tokens: Vec::new(),
            current: 0,
        };

        let statements: Vec<crate::stmt::StmtType> = parser.parse(scanned_tokens, self);
        if self.had_error {
            process::exit(65);
        }
        let interpreter = Interpreter::new();
        interpreter.interpret(statements);
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
