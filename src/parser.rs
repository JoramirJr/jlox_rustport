use std::str::FromStr;

use crate::expr::expr::{Binary, ExpressionGenericType, ExpressionType, Grouping, Literal, Unary};
use crate::token_type::*;

use jlox_rustport::ScanningParsingCommon;
struct Parser {
    tokens: Vec<Token<LiteralType>>,
    current: usize,
}

impl ScanningParsingCommon for Parser {
    fn error(line: &u32, message: &str) {
        Self::report(line, String::new(), message);
    }
    fn report(line: &u32, location: String, message: &str) {
        panic!("[line {}] Error {}: {}", line, location, message)
    }
}

impl Parser {
    fn new(tokens: Vec<Token<LiteralType>>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn expression(&mut self) -> ExpressionType<ExpressionGenericType> {
        Self::equality(self)
    }
    fn equality(&mut self) -> ExpressionType<ExpressionGenericType> {
        let mut expr = Self::comparison(self);
        while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous(self);
            let right = Self::comparison(self);
            if let Some(LiteralType::String(string_literal)) = operator.literal {
                expr = ExpressionType::BinaryExpr(Binary {
                    left: Box::new(expr),
                    operator: Token {
                        ttype: operator.ttype,
                        lexeme: operator.lexeme,
                        literal: Some(string_literal),
                        line: operator.line,
                    },
                    right: Box::new(right),
                });
            }
        }
        expr
    }
    fn comparison(&mut self) -> ExpressionType<ExpressionGenericType> {
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
            let operator = Self::previous(&self);
            let right = Self::term(self);

            if let Some(LiteralType::String(string_literal)) = operator.literal {
                expr = ExpressionType::BinaryExpr(Binary {
                    left: Box::new(expr),
                    operator: Token {
                        ttype: operator.ttype,
                        lexeme: operator.lexeme,
                        literal: Some(string_literal),
                        line: operator.line,
                    },
                    right: Box::new(right),
                })
            }
        }

        return expr;
    }
    fn term(&mut self) -> ExpressionType<ExpressionGenericType> {
        let mut expr = Self::factor(self);

        while Self::match_expr(self, &[TokenType::Minus, TokenType::Plus]) {
            let operator = Self::previous(self);
            let right = Self::factor(self);

            if let Some(LiteralType::String(string_literal)) = operator.literal {
                expr = ExpressionType::BinaryExpr(Binary {
                    left: Box::new(expr),
                    operator: Token {
                        ttype: operator.ttype,
                        lexeme: operator.lexeme,
                        literal: Some(string_literal),
                        line: operator.line,
                    },
                    right: Box::new(right),
                });
            }
        }

        return expr;
    }
    fn factor(&mut self) -> ExpressionType<ExpressionGenericType> {
        let mut expr = Self::unary(self);

        while Self::match_expr(self, &[TokenType::Slash, TokenType::Star]) {
            let operator = Self::previous(&self);
            let right = Self::unary(self);

            if let Some(LiteralType::String(string_literal)) = operator.literal {
                expr = ExpressionType::BinaryExpr(Binary {
                    left: Box::new(expr),
                    operator: Token {
                        ttype: operator.ttype,
                        lexeme: operator.lexeme,
                        literal: Some(string_literal),
                        line: operator.line,
                    },
                    right: Box::new(right),
                });
            }
        }

        return expr;
    }
    fn unary(&mut self) -> ExpressionType<ExpressionGenericType> {
        if Self::match_expr(self, &[TokenType::Bang, TokenType::Minus]) {
            let operator = Self::previous(&self);
            let right = Self::unary(self);

            if let Some(LiteralType::String(string_literal)) = operator.literal {
                return ExpressionType::UnaryExpr(Unary {
                    operator: Token {
                        ttype: operator.ttype,
                        lexeme: operator.lexeme,
                        literal: Some(string_literal),
                        line: operator.line,
                    },
                    right: Box::new(right),
                });
            }
        }
        return Self::primary(self);
    }
    fn primary(&mut self) -> ExpressionType<ExpressionGenericType> {
        if Self::match_expr(self, &[TokenType::False]) {
            return ExpressionType::LiteralExpr(Literal {
                value: Some(ExpressionGenericType::Token(TokenType::False)),
            });
        }
        if Self::match_expr(self, &[TokenType::True]) {
            return ExpressionType::LiteralExpr(Literal {
                value: Some(ExpressionGenericType::Token(TokenType::True)),
            });
        }
        if Self::match_expr(self, &[TokenType::Nil]) {
            return ExpressionType::LiteralExpr(Literal { value: None });
        }

        if Self::match_expr(self, &[TokenType::Number, TokenType::String]) {
            let previous = Self::previous(&self).literal;

            match previous {
                Some(LiteralType::String(_)) => {
                    return ExpressionType::LiteralExpr(Literal {
                        value: Some(ExpressionGenericType::Token(TokenType::String)),
                    });
                }
                Some(LiteralType::F32(_)) => {
                    return ExpressionType::LiteralExpr(Literal {
                        value: Some(ExpressionGenericType::Token(TokenType::Number)),
                    });
                }
                None => {}
            }
        }

        if Self::match_expr(self, &[TokenType::LeftParen]) {
            let expr = Self::expression(self);
            Self::consume(self, &TokenType::RightParen, "Expect ')' after expression");
            return ExpressionType::GroupingExpr(Grouping {
                expression: Box::new(ExpressionType::GroupingExpr(Grouping {
                    expression: Box::new(expr),
                })),
            });
        }
        Self::error(Self::peek(self), "Expect expression.");
        // ExpressionType::LiteralExpr(Literal { value: None })
    }
    fn consume(&mut self, t_type: &TokenType, message: &str) -> Token<LiteralType> {
        if !Self::check(self, t_type) {
            let next_token = Self::peek(self);
            if next_token.ttype == TokenType::Eof {
                Self::report(
                    &next_token.line,
                    String::from_str(" at end").unwrap(),
                    message,
                );
            } else {
                Self::report(
                    &next_token.line,
                    format!(" at '{}'", next_token.lexeme),
                    message,
                );
            }
        }
        Self::advance(self)
    }
    fn synchronize(&mut self) -> () {
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
    fn match_expr(&mut self, types: &[TokenType]) -> bool {
        let check = types.iter().any(|t| {
            if Self::check(self, t) {
                Self::advance(self);
                return true;
            } else {
                return false;
            }
        });
        if check {
            check
        } else {
            false
        }
    }
    fn check(&self, t_type: &TokenType) -> bool {
        if (Self::is_at_end(self)) {
            false
        } else {
            Self::peek(self).ttype == *t_type
        }
    }
    fn advance(&mut self) -> Token<LiteralType> {
        if Self::is_at_end(self) {
            self.current += 1;
        }
        Self::previous(self)
    }
    fn is_at_end(&self) -> bool {
        Self::peek(self).ttype == TokenType::Eof
    }
    fn peek(&self) -> Token<LiteralType> {
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token<LiteralType> {
        self.tokens[self.current - 1].clone()
    }
}
