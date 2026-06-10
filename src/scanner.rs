use std::io;
use std::io::Write;
use std::str::FromStr;

use crate::token_type::LiteralType;
use crate::token_type::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn scan_tokens(&mut self, source_file: String) -> Vec<Token> {
        self.source = source_file;

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let line = self.line;

        self.tokens.push(Token {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: Some(LiteralType::Nil),
            line: line,
        });
        let tokens = self.tokens.clone();
        tokens
    }
    pub fn report(line: &u32, location: String, message: &str) -> () {
        let err_msg = format!("[line {}] Error {}: {}", line, location, message);
        let mut err_out_handler = io::stderr();
        let _ = err_out_handler.write_all(err_msg.as_bytes());
    }
    pub fn error(line: &u32, message: &str) {
        Self::report(line, String::new(), message);
    }
    pub fn scan_token(&mut self) -> () {
        let c: u8 = self.advance();

        if c != b'\0' {
            match c {
                b'(' => self.add_token(TokenType::LeftParen, None),
                b')' => self.add_token(TokenType::RightParen, None),
                b'{' => self.add_token(TokenType::LeftBrace, None),
                b'}' => self.add_token(TokenType::RightBrace, None),
                b',' => self.add_token(TokenType::Comma, None),
                b'.' => self.add_token(TokenType::Dot, None),
                b'-' => self.add_token(TokenType::Minus, None),
                b'+' => self.add_token(TokenType::Plus, None),
                b';' => self.add_token(TokenType::Semicolon, None),
                b'*' => self.add_token(TokenType::Star, None),
                b'!' => self.match_token_sequence(b'!', b'='),
                b'=' => self.match_token_sequence(b'=', b'='),
                b'<' => self.match_token_sequence(b'<', b'='),
                b'>' => self.match_token_sequence(b'>', b'='),
                b'/' => self.match_token_sequence(b'/', b'/'),
                b' ' => {}
                b'\r' => {}
                b'\t' => {}
                b'\n' => self.line += 1,
                b'"' => self.string(),
                _ => {
                    if Self::is_digit(c) {
                        self.number();
                    } else if Self::is_alpha(c) {
                        self.identifier();
                    } else {
                        Scanner::error(&self.line, "Unexpected character.")
                    }
                }
            }
        }
    }
    pub fn advance(&mut self) -> u8 {
        self.current = self.current + 1;
        self.source.as_bytes()[self.current - 1]
    }
    pub fn is_alpha(c: u8) -> bool {
        (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
    }
    pub fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source.as_bytes()[self.start..self.current];

        let ttype = match text {
            b"and" => TokenType::And,
            b"class" => TokenType::Class,
            b"else" => TokenType::Else,
            b"false" => TokenType::False,
            b"for" => TokenType::For,
            b"fun" => TokenType::Fun,
            b"if" => TokenType::If,
            b"nil" => TokenType::Nil,
            b"or" => TokenType::Or,
            b"print" => TokenType::Print,
            b"return" => TokenType::Return,
            b"super" => TokenType::Super,
            b"this" => TokenType::This,
            b"true" => TokenType::True,
            b"var" => TokenType::Var,
            b"while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(ttype, Some(LiteralType::Nil))
    }
    pub fn is_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }
    pub fn is_alphanumeric(peeked_c: u8) -> bool {
        Self::is_alpha(peeked_c) || Self::is_digit(peeked_c)
    }
    pub fn match_token(&mut self, expected: u8) -> bool {
        if Self::is_at_end(&self) {
            return false;
        } else if self.source.as_bytes()[self.current] != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    pub fn match_token_sequence(&mut self, case: u8, expected: u8) {
        let match_sequence = self.match_token(expected);

        if case == b'!' {
            let ttype = if match_sequence {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            self.add_token(ttype, Some(LiteralType::Nil))
        } else if case == b'=' {
            let ttype = if match_sequence {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            self.add_token(ttype, Some(LiteralType::Nil))
        } else if case == '<' {
            let ttype = if match_sequence {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            self.add_token(ttype, Some(LiteralType::Nil))
        } else if case == '>' {
            let ttype = if match_sequence {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            self.add_token(ttype, Some(LiteralType::Nil))
        } else if case == b'/' {
            if match_sequence {
                return true;

                while self.peek() != '\n' && !self.is_at_end() {
                    let _ = self.advance();
                }
            } else {
                self.add_token(TokenType::Slash, Some(LiteralType::Nil));
            };
        }
    }
    pub fn add_token(&mut self, ttype: TokenType, literal: Option<LiteralType>) {
        if let Some(text) = self.source.get(self.start..self.current) {
            self.tokens.push(Token {
                ttype,
                lexeme: text.to_string(),
                literal,
                line: self.line,
            });
        }
    }
    pub fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        } else {
            return self.source.u8s().nth(self.current).unwrap();
        }
    }
    pub fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let float_number: f32 = f32::from_str(
            &self
                .source
                .as_bytes()
                .skip(self.start)
                .take(self.current - self.start)
                .collect::<String>(),
        )
        .ok()
        .unwrap();

        self.add_token(TokenType::Number, Some(LiteralType::F32(float_number)));
    }
    pub fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Scanner::error(&self.line, "Unterminated string");
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(LiteralType::String(value)))
    }
    pub fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
