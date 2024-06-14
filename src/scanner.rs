use std::fmt::Debug;

use crate::token_type::Token;
use crate::token_type::TokenType;

struct Scanner<T: Debug> {
    source: String,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: u32,
}

impl<T: Debug> Scanner<T> {
    fn scan_tokens(mut self) -> Vec<Token<T>> {
        while !Self::is_at_end(&self) {
            self.start = self.current;
            Self::scan_token(&mut self);
        }

        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
        self.tokens
    }
    fn scan_token(&mut self) {
        let c: Option<char> = Self::advance(&mut self);

        let call_add_token = Self::set_self_for_add_token(self);

        if c != None {
            match c.unwrap() {
                '(' => call_add_token(TokenType::LEFT_PAREN),
                ')' => call_add_token(TokenType::RIGHT_PAREN),
                '{' => call_add_token(TokenType::LEFT_BRACE),
                '}' => call_add_token(TokenType::RIGHT_BRACE),
                ',' => call_add_token(TokenType::COMMA),
                '.' => call_add_token(TokenType::DOT),
                '-' => call_add_token(TokenType::MINUS),
                '+' => call_add_token(TokenType::PLUS),
                ';' => call_add_token(TokenType::SEMICOLON),
                '*' => call_add_token(TokenType::STAR),
            }
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn set_self_for_add_token(self) -> fn(TokenType) {
        let self_value: Scanner<T> = self;
        Self::call_add_token
    }
    fn call_add_token(ttype: TokenType) {
        Self::add_token(self_value, ttype, None);
    }
    fn add_token(&mut self, ttype: TokenType, literal: Option<T>) {
        //not sure if 'get' will bring me the intended substring
        let text = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();
        self.tokens.push(Token {
            ttype,
            lexeme: text,
            literal,
            line: self.line,
        });
    }
    fn is_at_end(&self) -> bool {
        &self.current >= &self.source.len()
    }
}
