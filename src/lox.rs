use crate::token_type::{Token, TokenType};
use crate::{ast_printer, expr, parser, scanner};
use std::{
    fs,
    io::{self, Write},
    process,
    str::FromStr,
};

use expr::ExpressionType;
use parser::Parser;
use scanner::Scanner;

pub struct Lox {
    pub args: Vec<String>,
    pub had_error: bool,
}

impl Lox {
    pub fn start(mut self) {
        let args_length = self.args.len();
        if args_length < 1 || args_length > 2 {
            println!("Usage: jlox [script]");
            process::exit(64);
        } else if args_length == 2 {
            Self::run_file(&mut self);
        }
        //  else if args_length == 2 {
        //     Self::run_prompt(self);
        // }
    }
    pub fn run_file(&self) {
        let file = fs::read_to_string(&self.args[1]).expect("File reading successful");
        let scanner: Scanner = Scanner {
            source: file,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        let scanned_tokens = scanner.scan_tokens();
        let mut parser = Parser::new(scanned_tokens);
        let expr = parser.parse();

        if self.had_error {
            process::exit(65);
        }

        println!("expr: {:?}", expr);

        match expr {
            Ok(expr) => {
                // if let ExpressionType::BinaryExpr(sub_type) = expr {
                //     println!(
                //         "{:?}",
                //         ast_printer::AstPrinter::print(&ExpressionType::BinaryExpr(sub_type))
                //     )
                // }
                if let ExpressionType::GroupingExpr(sub_type) = expr {
                    println!(
                        "Print: {:?}",
                        ast_printer::AstPrinter::print(&ExpressionType::GroupingExpr(sub_type))
                    )
                }
            }
            Err(_) => {}
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
    pub fn error(token: Token, message: &str) -> String {
        if token.ttype == TokenType::Eof {
            Self::report(&token.line, String::from_str(" at end").unwrap(), message)
        } else {
            Self::report(&token.line, format!(" at '{}'", token.lexeme), message)
        }
    }
    pub fn report(line: &u32, location: String, message: &str) -> String {
        format!("[line {}] Error {}: {}", line, location, message)
    }
}
