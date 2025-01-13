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
        while Self::match_expr([TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous();
            let right = Self::comparison();
            expr = expr
        }
    }
    fn comparison() {}
    fn match_expr(types: &[TokenType]) {}
}
