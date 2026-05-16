use std::io;
use std::io::Write;
use std::str::FromStr;

use crate::token_type::LiteralType;
use crate::token_type::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn scan_tokens(&mut self, source_file: String) -> Vec<Token> {
        self.source = source_file.chars().collect();

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
        let c = self.advance();

        if c != None {
            match c.unwrap() {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => self.match_token_sequence('!', '='),
                '=' => self.match_token_sequence('=', '='),
                '<' => self.match_token_sequence('<', '='),
                '>' => self.match_token_sequence('>', '='),
                '/' => self.match_token_sequence('/', '/'),
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => self.string(),
                _ => {
                    if Self::is_digit(c.unwrap()) {
                        self.number();
                    } else if Self::is_alpha(c.unwrap()) {
                        self.identifier();
                    } else {
                        Scanner::error(&self.line, "Unexpected character.")
                    }
                }
            }
        }
    }
    pub fn advance(&mut self) -> Option<&char> {
        self.current = self.current + 1;
        self.source.get(self.current - 1)
    }
    pub fn is_alpha(c: &char) -> bool {
        (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || *c == '_'
    }
    pub fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).unwrap();

        let ttype = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(ttype, Some(LiteralType::Nil))
    }
    pub fn is_digit(c: &char) -> bool {
        *c >= '0' && *c <= '9'
    }
    pub fn is_alphanumeric(peeked_c: &char) -> bool {
        Self::is_alpha(peeked_c) || Self::is_digit(peeked_c)
    }
    pub fn match_token(&mut self, expected: char) -> bool {
        if Self::is_at_end(&self) {
            return false;
        } else if self.source.get(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    pub fn match_token_sequence(&mut self, case: char, expected: char) {
        let match_sequence = self.match_token(expected);

        if case == '!' {
            let ttype = if match_sequence {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            self.add_token(ttype, Some(LiteralType::Nil))
        } else if case == '=' {
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
        } else if case == '/' {
            if match_sequence {
                while self.peek() != &'\n' && !self.is_at_end() {
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
    pub fn peek(&self) -> &char {
        if self.is_at_end() {
            return &'\0';
        } else {
            return self.source.get(self.current).unwrap();
        }
    }
    pub fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == &'.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let float_number: f32 = f32::from_str(self.source.get(self.start..self.current).unwrap())
            .ok()
            .unwrap();

        self.add_token(TokenType::Number, Some(LiteralType::F32(float_number)));
    }
    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Scanner::error(&self.line, "Unterminated string");
        }

        self.advance();

        let value: String = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        self.add_token(TokenType::String, Some(LiteralType::String(value)))
    }
    pub fn peek_next(&self) -> &char {
        if self.current + 1 >= self.source.len() {
            &'\0'
        } else {
            self.source.get(self.current + 1).unwrap()
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
