use std::str::FromStr;

// use crate::expr::expr::{Binary, ExpressionType, Grouping, Literal, Unary};
use crate::token_type::*;

use jlox_rustport::ScanningParsingCommon;
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl ScanningParsingCommon for Parser {
    fn report(line: &u32, location: String, message: &str) {
        panic!("[line {}] Error {}: {}", line, location, message)
    }
    fn error(_: &u32, _: &str) -> () {}
}

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Parser { tokens, current: 0 }
//     }
//     pub fn parse(&mut self) -> ExpressionType {
//         Self::expression(self)
//     }
//     pub fn error(token: Token, message: &str) {
//         if token.ttype == TokenType::Eof {
//             Self::report(&token.line, String::from_str(" at end").unwrap(), message);
//         } else {
//             Self::report(&token.line, format!(" at '{}'", token.lexeme), message);
//         }
//     }
//     pub fn expression(&mut self) -> ExpressionType {
//         Self::equality(self)
//     }
//     pub fn equality(&mut self) -> ExpressionType {
//         let mut expr = Self::comparison(self);
//         while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
//             let operator = Self::previous(self);
//             let right = Self::comparison(self);
//             if let LiteralType::String(string_literal) = operator.literal {
//                 expr = ExpressionType::BinaryExpr(Binary {
//                     left: Box::new(expr),
//                     operator: Token {
//                         ttype: operator.ttype,
//                         lexeme: operator.lexeme,
//                         literal: LiteralType::String(string_literal),
//                         line: operator.line,
//                     },
//                     right: Box::new(right),
//                 });
//             }
//         }
//         expr
//     }
//     pub fn comparison(&mut self) -> ExpressionType {
//         let mut expr = Self::term(self);

//         while Self::match_expr(
//             self,
//             &[
//                 TokenType::Greater,
//                 TokenType::GreaterEqual,
//                 TokenType::Less,
//                 TokenType::LessEqual,
//             ],
//         ) {
//             let operator = Self::previous(&self);
//             let right = Self::term(self);

//             if let LiteralType::String(string_literal) = operator.literal {
//                 expr = ExpressionType::BinaryExpr(Binary {
//                     left: Box::new(expr),
//                     operator: Token {
//                         ttype: operator.ttype,
//                         lexeme: operator.lexeme,
//                         literal: LiteralType::String(string_literal),
//                         line: operator.line,
//                     },
//                     right: Box::new(right),
//                 })
//             }
//         }

//         return expr;
//     }
//     pub fn term(&mut self) -> ExpressionType {
//         let mut expr = Self::factor(self);

//         while Self::match_expr(self, &[TokenType::Minus, TokenType::Plus]) {
//             let operator = Self::previous(self);
//             let right = Self::factor(self);

//             if let LiteralType::String(string_literal) = operator.literal {
//                 expr = ExpressionType::BinaryExpr(Binary {
//                     left: Box::new(expr),
//                     operator: Token {
//                         ttype: operator.ttype,
//                         lexeme: operator.lexeme,
//                         literal: LiteralType::String(string_literal),
//                         line: operator.line,
//                     },
//                     right: Box::new(right),
//                 });
//             }
//         }

//         return expr;
//     }
//     pub fn factor(&mut self) -> ExpressionType {
//         let mut expr = Self::unary(self);

//         while Self::match_expr(self, &[TokenType::Slash, TokenType::Star]) {
//             let operator = Self::previous(&self);
//             let right = Self::unary(self);

//             if let Some(LiteralType::String(string_literal)) = operator.literal {
//                 expr = ExpressionType::BinaryExpr(Binary {
//                     left: Box::new(expr),
//                     operator: Token {
//                         ttype: operator.ttype,
//                         lexeme: operator.lexeme,
//                         literal: Some(string_literal),
//                         line: operator.line,
//                     },
//                     right: Box::new(right),
//                 });
//             }
//         }

//         return expr;
//     }
//     pub fn unary(&mut self) -> ExpressionType {
//         if Self::match_expr(self, &[TokenType::Bang, TokenType::Minus]) {
//             let operator = Self::previous(&self);
//             let right = Self::unary(self);

//             if let Some(LiteralType::String(string_literal)) = operator.literal {
//                 return ExpressionType::UnaryExpr(Unary {
//                     operator: Token {
//                         ttype: operator.ttype,
//                         lexeme: operator.lexeme,
//                         literal: Some(string_literal),
//                         line: operator.line,
//                     },
//                     right: Box::new(right),
//                 });
//             }
//         }
//         return Self::primary(self);
//     }
//     pub fn primary(&mut self) -> ExpressionType {
//         if Self::match_expr(self, &[TokenType::False]) {
//             return ExpressionType::LiteralExpr(Literal {
//                 value: LiteralType::Bool(false),
//             });
//         }
//         if Self::match_expr(self, &[TokenType::True]) {
//             return ExpressionType::LiteralExpr(Literal {
//                 value: LiteralType::Bool(true),
//             });
//         }
//         if Self::match_expr(self, &[TokenType::Nil]) {
//             return ExpressionType::LiteralExpr(Literal {
//                 value: LiteralType::Nil,
//             });
//         }

//         if Self::match_expr(self, &[TokenType::Number, TokenType::String]) {
//             return ExpressionType::LiteralExpr(Literal {
//                 value: Self::previous(&self).literal,
//             });
//         }

//         if Self::match_expr(self, &[TokenType::LeftParen]) {
//             let expr = Self::expression(self);
//             Self::consume(self, &TokenType::RightParen, "Expect ')' after expression");
//             return ExpressionType::GroupingExpr(Grouping {
//                 expression: Box::new(ExpressionType::GroupingExpr(Grouping {
//                     expression: Box::new(expr),
//                 })),
//             });
//         }
//         Self::error(Self::peek(self), "Expect expression.");
//         ExpressionType::LiteralExpr(Literal {
//             value: LiteralType::Nil,
//         })
//     }
//     pub fn consume(&mut self, t_type: &TokenType, message: &str) -> Token {
//         if !Self::check(self, t_type) {
//             let next_token = Self::peek(self);
//             Self::error(next_token, message);
//         }
//         Self::advance(self)
//     }
//     pub fn synchronize(&mut self) -> () {
//         Self::advance(self);

//         while !Self::is_at_end(self) {
//             if Self::previous(self).ttype == TokenType::Semicolon {
//                 return;
//             }
//             match Self::peek(self).ttype {
//                 TokenType::Class => return,
//                 TokenType::For => return,
//                 TokenType::Fun => return,
//                 TokenType::If => return,
//                 TokenType::Print => return,
//                 TokenType::Return => return,
//                 TokenType::Var => return,
//                 TokenType::While => return,
//                 _ => {}
//             }
//             Self::advance(self);
//         }
//     }
//     pub fn match_expr(&mut self, types: &[TokenType]) -> bool {
//         let check = types.iter().any(|t| {
//             if Self::check(self, t) {
//                 Self::advance(self);
//                 return true;
//             } else {
//                 return false;
//             }
//         });
//         if check {
//             check
//         } else {
//             false
//         }
//     }
//     pub fn check(&self, t_type: &TokenType) -> bool {
//         if Self::is_at_end(self) {
//             false
//         } else {
//             Self::peek(self).ttype == *t_type
//         }
//     }
//     pub fn advance(&mut self) -> Token {
//         if Self::is_at_end(self) {
//             self.current += 1;
//         }
//         Self::previous(self)
//     }
//     pub fn is_at_end(&self) -> bool {
//         Self::peek(self).ttype == TokenType::Eof
//     }
//     pub fn peek(&self) -> Token {
//         self.tokens[self.current].clone()
//     }
//     pub fn previous(&self) -> Token {
//         self.tokens[self.current - 1].clone()
//     }
// }
