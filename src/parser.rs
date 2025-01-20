use crate::token_type::*;

struct Parser {
    tokens: Vec<Token<String>>,
    current: u16,
}

impl Parser {
    fn new(tokens: Vec<Token<String>>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn expression() -> fn() {
        Self::equality()
    }
    fn equality() {
        let mut expr = Self::comparison();
        while Self::match_expr([TokenType::BangEqual, TokenType::EqualEqual] ) {
            let operator = Self::previous();
            let right = Self::comparison();
            expr = expr
        }
    }
    fn comparison() {}
    fn match_expr(types: &[TokenType]) -> bool {
        types.iter().any(|t| { 
            if Self::check(t) {
                Self::advance();
                return true;
            } else {
                return false;
            }
         });
         false
    }
    fn check(t_type: &TokenType) -> bool {
        if(Self::is_at_end()) {
            false
        } else {
            Self::peek().type == t_type
        }
    }
    fn advance() {
        if(Self::is_at_end()) {
            self.current += 1;
        }
        return Self::previous()
    }
    fn is_at_end() {}
    fn peek() {}
    fn previous(){}
}
