use crate::token_type::*;
use crate::expr::expr::Binary;

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
        let mut expr = Self::comparison();
        while Self::match_expr(self, [TokenType::BangEqual, TokenType::EqualEqual] ) {
            let operator = Self::previous(self);
            let right = Self::comparison();
            expr = Binary { left: expr, operator, right: right }
        }
        expr
    }
    fn comparison() -> Binary {
        
    }
    fn match_expr(&mut self, types: [TokenType; 2]) -> bool {
        types.iter().any(|t| { 
            if Self::check(self, t) {
                Self::advance(self);
                return true;
            } else {
                return false;
            }
         });
         false
    }
    fn check(&self, t_type: &TokenType) -> bool {
        if(Self::is_at_end(self)) {
            false
        } else {
            Self::peek(self).ttype == *t_type
        }
    }
    fn advance(&mut self) -> Token<String> {
        if Self::is_at_end(self)  {
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
