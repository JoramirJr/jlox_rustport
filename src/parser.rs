use std::sync::{LazyLock, Mutex};

use crate::expr::{Binary, ExpressionType, Grouping, Literal, Unary};
use crate::lox::{Lox, LOX_SINGLETON};
use crate::stmt::StmtType;
use crate::{stmt, token_type::*};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
pub struct ParseError(String);

pub static PARSER_SINGLETON: LazyLock<Mutex<Parser>> = LazyLock::new(|| {
    Mutex::new(Parser {
        tokens: Vec::new(),
        current: 0,
    })
});

impl Parser {
    pub fn parse(scanned_tokens: Vec<Token>) -> Option<ExpressionType> {
        let parser_singleton = PARSER_SINGLETON.lock();

        match parser_singleton {
            Ok(mut parser) => {
                let mut statements: Vec<()> = Vec::new();

                while !Self::is_at_end(&parser) {
                    statements.push(Self::statement());
                }
                statements
            }
            Err(err) => {
                panic!("Parser singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    fn statement() -> StmtType {
        if Self::match_expr(&mut self, &[TokenType::Print]) {
            Self::printStatement()
        }
        Self::expressionStatement()
    }
    fn printStatement() -> () {}
    fn expressionStatement() -> () {}
    pub fn expression(&mut self) -> Result<ExpressionType, ParseError> {
        Self::equality(self)
    }
    pub fn equality(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::comparison(self);

        while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        expr
    }
    pub fn comparison(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::term(self);

        while Self::match_expr(
            self,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        ) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn term(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::factor(self);
        while Self::match_expr(self, &[TokenType::Minus, TokenType::Plus]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn factor(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::unary(self);

        while Self::match_expr(self, &[TokenType::Slash, TokenType::Star]) {
            let operator = Self::previous(&self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn unary(&mut self) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::Bang, TokenType::Minus]) {
            let operator = Self::previous(&self);
            let right = Self::unary(self);

            match right {
                Ok(ok_response) => {
                    return Ok(ExpressionType::UnaryExpr(Unary {
                        operator: Token {
                            ttype: operator.ttype,
                            lexeme: operator.lexeme,
                            literal: operator.literal,
                            line: operator.line,
                        },
                        right: Box::new(ok_response),
                    }));
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        return Self::primary(self);
    }
    pub fn primary(&mut self) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::False]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Bool(false),
            }));
        }
        if Self::match_expr(self, &[TokenType::True]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Bool(true),
            }));
        }
        if Self::match_expr(self, &[TokenType::Nil]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Nil,
            }));
        }

        if Self::match_expr(self, &[TokenType::Number, TokenType::String]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: Self::previous(&self).literal,
            }));
        }

        if Self::match_expr(self, &[TokenType::LeftParen]) {
            let expr = Self::expression(self);

            match expr {
                Ok(ok_response) => {
                    Self::consume(self, &TokenType::RightParen, "Expect ')' after expression")?;
                    return Ok(ExpressionType::GroupingExpr(Grouping {
                        expression: Box::new(ExpressionType::GroupingExpr(Grouping {
                            expression: Box::new(ok_response),
                        })),
                    }));
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        Self::error(Self::peek(self), "Expect expression.");

        return Ok(ExpressionType::LiteralExpr(Literal {
            value: LiteralType::Nil,
        }));
    }
    pub fn consume(&mut self, t_type: &TokenType, message: &str) -> Result<Token, ParseError> {
        if !Self::check(self, t_type) {
            let next_token = Self::peek(self);
            return Err(Self::error(next_token, message));
        }
        Ok(Self::advance(self))
    }
    pub fn synchronize(&mut self) -> () {
        Self::advance(self);

        while !Self::is_at_end(self) {
            if Self::previous(self).ttype == TokenType::Semicolon {
                return;
            }
            match Self::peek(self).ttype {
                TokenType::Class => return,
                TokenType::For => return,
                TokenType::Fun => return,
                TokenType::If => return,
                TokenType::Print => return,
                TokenType::Return => return,
                TokenType::Var => return,
                TokenType::While => return,
                _ => {}
            }
            Self::advance(self);
        }
    }
    pub fn match_expr(&mut self, types: &[TokenType]) -> bool {
        types.iter().any(|t| {
            if Self::check(self, t) {
                Self::advance(self);
                return true;
            } else {
                return false;
            }
        })
    }
    pub fn check(&self, t_type: &TokenType) -> bool {
        if Self::is_at_end(self) {
            false
        } else {
            Self::peek(self).ttype == *t_type
        }
    }
    pub fn advance(&mut self) -> Token {
        if !Self::is_at_end(self) {
            self.current += 1;
        }
        Self::previous(self)
    }
    pub fn is_at_end(&self) -> bool {
        Self::peek(self).ttype == TokenType::Eof
    }
    pub fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    pub fn error(token: Token, message: &str) -> ParseError {
        Lox::error(token, message);
        ParseError("".to_string())
    }
}
