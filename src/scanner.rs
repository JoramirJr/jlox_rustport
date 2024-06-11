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

        

        if c != None {
            match c.unwrap() {
                '(' => add_token(),
                ')' => add_token(),
                '{' => add_token(),
                '}' => add_token(),
                ',' => add_token(),
                '.' => add_token(),
                '-' => add_token(),
                '+' => add_token(),
                ';' => add_token(),
                '*' => add_token(),
            }
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn call_add_token(mut self, ttype: TokenType) {
        Self::add_token(&mut self, ttype, None);
    }
    fn add_token(&mut self, ttype: TokenType, literal: Option<T>) -> fn() {
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
