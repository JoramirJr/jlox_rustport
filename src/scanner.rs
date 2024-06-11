use std::arch::x86_64::_addcarryx_u32;
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
            Self::scan_token();
        }

        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
        self.tokens
    }
    fn scan_token(mut self) {
        let c: Option<char> = Self::advance(self);
        match c.is_some() {
            '(' => _add_token(),
            ')' => _add_token(),
            '{' => _add_token(),
            '}' => _add_token(),
            ',' => _add_token(),
            '.' => _add_token(),
            '-' => _add_token(),
            '+' => _add_token(),
            ';' => _add_token(),
            '*' => _add_token(),
        }
    }
    fn advance(mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn is_at_end(&self) -> bool {
        &self.current >= &self.source.len()
    }
}
