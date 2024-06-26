use std::str::FromStr;

use crate::token_type::Token;
use crate::token_type::TokenType;
use crate::Main;
struct Scanner<T> {
    source: String,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: u32,
}

impl<T> Scanner<T> {
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
        let c: Option<char> = Self::advance(self);

        let mut call_add_token = |ttype: TokenType| {
            Self::add_token(self, ttype, None);
        };

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
                '!' => Self::match_token_sequence(self, '!', '='),
                '=' => Self::match_token_sequence(self, '=', '='),
                '<' => Self::match_token_sequence(self, '<', '='),
                '>' => Self::match_token_sequence(self, '>', '='),
                '/' => Self::match_token_sequence(self, '/', '/'),
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => Self::string(self),
                _ => {
                    if Self::is_digit('c') {
                    } else {
                        Main::error(&self.line, "Unexpected character.")
                    }
                }
            }
        }
    }
    fn match_token(&mut self, expected: char) -> bool {
        if Self::is_at_end(&self) {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    fn match_token_sequence(&mut self, case: char, expected: char) {
        let match_sequence = Self::match_token(self, expected);

        if case == '!' {
            let ttype = if match_sequence {
                TokenType::BANG_EQUAL
            } else {
                TokenType::BANG
            };
            Self::add_token(self, ttype, None)
        } else if case == '=' {
            let ttype = if match_sequence {
                TokenType::EQUAL_EQUAL
            } else {
                TokenType::EQUAL
            };
            Self::add_token(self, ttype, None)
        } else if case == '<' {
            let ttype = if match_sequence {
                TokenType::LESS_EQUAL
            } else {
                TokenType::LESS
            };
            Self::add_token(self, ttype, None)
        } else if case == '>' {
            let ttype = if match_sequence {
                TokenType::GREATER_EQUAL
            } else {
                TokenType::GREATER
            };
            Self::add_token(self, ttype, None)
        } else if case == '/' {
            if match_sequence {
                while Self::peek(&self) != '\n' && Self::is_at_end(&self) {
                    let _ = Self::advance(self);
                }
            } else {
                Self::add_token(self, TokenType::SLASH, None);
            };
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn peek(&self) -> char {
        if Self::is_at_end(&self) {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
    fn number(&mut self) {
        while Self::is_digit(Self::peek(&self)) {
            Self::advance(self);
        }

        if Self::peek(&self) == '.' && Self::is_digit(Self::peek_next(&self)) {
            Self::advance(self);

            while Self::is_digit(Self::peek(&self)) {
                Self::advance(self);
            }
        }

        let float_number: Option<f32> = f32::from_str(self.source.get(self.start..self.current).unwrap())
            .ok()

        Self::add_token(self, TokenType::NUMBER, float_number);
    }
    fn string(&mut self) {
        while Self::peek(&self) != '"' && !Self::is_at_end(&self) {
            if Self::peek(&self) != '\n' {
                self.line += 1;
                Self::advance(self);
            }
        }
        if Self::is_at_end(&self) {
            Main::error(&self.line, "Unterminated string");
        }

        Self::advance(self);

        let value: String = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        Self::add_token(self, TokenType::STRING, Some(value))
    }
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
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
