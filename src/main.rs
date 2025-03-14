use std::{env, fs, io, io::Write, process};
use jlox_rustport::{ast_printer, expr, parser, scanner, token_type};

use expr::{Binary, ExpressionType, Grouping, Literal, Unary};
use token_type::{LiteralType, Token, TokenType};
use parser::Parser;
use scanner::Scanner;
struct Main {
    args: Vec<String>,
    had_error: bool,
}

 impl Main {
     fn main(mut self) {
        //   let args_length = self.args.len();
        //   if args_length < 2 || args_length > 3 {
        //       println!("Usage: jlox [script]");
        //       process::exit(64);
        //   } else if args_length == 3 {
        //       Self::run_file(&mut self);
        //   } else if args_length == 2 {
        //       Self::run_prompt(self);
        //   }
         let expr = Binary {
             left: Box::new(ExpressionType::UnaryExpr(Unary {
                 operator: Token {
                     lexeme: "-".to_string(),
                     literal: LiteralType::Nil,
                     line: 1,
                     ttype: TokenType::Minus,
                 },
                 right: Box::new(ExpressionType::LiteralExpr(Literal {
                     value: LiteralType::F32(123 as f32),
                 })),
             })),
             operator: Token {
                 lexeme: "*".to_string(),
                 line: 1,
                 literal: LiteralType::Nil,
                 ttype: TokenType::Star,
             },
             right: Box::new(ExpressionType::GroupingExpr(Grouping {
                 expression: Box::new(ExpressionType::LiteralExpr(Literal {
                     value: LiteralType::F32(45.67),
                 })),
             })),
         };

         println!(
             "{:?}",
             ast_printer::AstPrinter::print(ExpressionType::BinaryExpr(expr))
         )
     }
     fn run_file(&self) {
         if self.had_error {
             process::exit(65);
         } else {
             let file = fs::read_to_string(&self.args[2]).expect("File reading successful");
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
          source.split(" ").for_each(|token| {
              let _ = std_out_handler.write_all(token.as_bytes());
          });
     }
 }

fn main() {
    let main = Main {
        args: env::args().collect(),
        had_error: false,
    };
     main.main();
}
