mod expr;
mod parser;
mod scanner;
mod token_type;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use parser::Parser;
use scanner::Scanner;
use token_type::LiteralType;

struct Main {
    args: Vec<String>,
    had_error: bool,
}

impl Main {
    fn main(mut self) {
        let args_length = self.args.len();
        if args_length < 2 || args_length > 3 {
            println!("Usage: jlox [script]");
            process::exit(64);
        } else if args_length == 3 {
            Self::run_file(&mut self);
        } else if args_length == 2 {
            Self::run_prompt(self);
        }
    }
    fn run_file(&self) {
        if self.had_error {
            process::exit(65);
        } else {
            let file = fs::read_to_string(&self.args[2]).expect("File reading successful");
            let scanner: Scanner<LiteralType> = Scanner {
                source: file,
                tokens: None,
                start: 0,
                current: 0,
                line: 1,
            };
            let scanned_tokens = scanner.scan_tokens();
            let mut parser = Parser::new(scanned_tokens);
            let expr = parser.parse();
            println!("{:?}", expr);
        }
    }

    fn run_prompt(mut self) -> io::Error {
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

    fn run(source: &String, std_out_handler: &mut io::Stdout) {
        // source.split(" ").for_each(|token| {
        //     let _ = std_out_handler.write_all(token.as_bytes());
        // });
    }
}

fn main() {
    let main = Main {
        args: env::args().collect(),
        had_error: false,
    };
    main.main();
}
