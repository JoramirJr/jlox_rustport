use crate::expr::expr::{Binary, Grouping, Literal, Unary};
use crate::token_type::*;

struct Parser {
    tokens: Vec<Token<String>>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token<String>>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn expression() -> fn() {
        Self::equality()
    }
    fn equality(&mut self) -> Binary {
        let mut expr = Self::comparison(self);
        while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous(self);
            let right = Self::comparison(self);
            expr = Binary {
                left: expr,
                operator,
                right: right,
            }
        }
        expr
    }
    fn comparison(&mut self) -> Binary {
        let expr = Self::term();

        while Self::match_expr(
            &mut self,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        ) {
            let operator = Self::previous(&self);
            let right = Self::term();
            let expr = Binary { left: expr, operator: operator, right: right }
        }

        return expr;
    }
    fn term() {
        let expr = Self::factor();

        while Self::match_expr(&mut self, &[TokenType::Minus, TokenType::Plus]) {
            let operator = Self::previous(&self);
            let right = Self::factor();
            let expr = Binary { left: expr, operator: operator, right: right }
        }

        return expr;

    }
    fn factor() {
        let expr = Self::unary();

        while Self::match_expr(&mut self, &[TokenType::Slash, TokenType::Star]) {
            let operator = Self::previous(&self);
            let right = Self::unary();
            let expr = Binary { left: expr, operator: operator, right: right }
        }

        return expr;

    }
    fn unary(){
        if Self::match_expr(&mut self, &[TokenType::Bang, TokenType::Minus]) {
            let operator = Self::previous(&self);
            let expr = Self::unary();
            return Unary { operator: operator, right: right } 
        }
        return; Self::primary();
    }
    fn primary(){
        if Self::match_expr(&mut self, &[TokenType::False]) {
            Literal { value: false }
        }
        if Self::match_expr(&mut self, &[TokenType::True]) {
            Literal { value: true }
        }
        if Self::match_expr(&mut self, &[TokenType::Nil]) {
            Literal { value: () }
        }

        if Self::match_expr(&mut self, &[TokenType::Number, TokenType::String]) {
            Literal { value: Self::previous(&self).literal }
        }

        if Self::match_expr(&mut self, &[TokenType::LeftParen]) {
            let expr = Self::expression();
            Self::consume(TokenType::RightParen, "Expect ')' after expression");
            Grouping { expression: expr }
        }
    }
    fn consume (){}
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
    fn advance(&mut self) -> Token<String> {
        if Self::is_at_end(self) {
            self.current += 1;
        }
        Self::previous(self)
    }
    fn is_at_end(&self) -> bool {
        Self::peek(self).ttype == TokenType::Eof
    }
    fn peek(&self) -> Token<String> {
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token<String> {
        self.tokens[self.current - 1].clone()
    }
}
