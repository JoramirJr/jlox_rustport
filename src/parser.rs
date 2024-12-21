use crate::token_type::Token;

struct Parser {
    tokens: Vec<Token>,
    current: u16,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn expression() -> fn() {
        Self::equality()
    }
    fn equality() {
        
    }
}
