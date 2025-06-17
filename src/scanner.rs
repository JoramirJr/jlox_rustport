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

        while !Self::is_at_end(&self) {
            self.start = self.current;
            Self::scan_token(self);
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
        let c: Option<char> = Self::advance(self);

        if c != None {
            match c.unwrap() {
                '(' => Self::add_token(self, TokenType::LeftParen, None),
                ')' => Self::add_token(self, TokenType::RightParen, None),
                '{' => Self::add_token(self, TokenType::LeftBrace, None),
                '}' => Self::add_token(self, TokenType::RightBrace, None),
                ',' => Self::add_token(self, TokenType::Comma, None),
                '.' => Self::add_token(self, TokenType::Dot, None),
                '-' => Self::add_token(self, TokenType::Minus, None),
                '+' => Self::add_token(self, TokenType::Plus, None),
                ';' => Self::add_token(self, TokenType::Semicolon, None),
                '*' => Self::add_token(self, TokenType::Star, None),
                '!' => Self::match_token_sequence(self, '!', Some('=')),
                '=' => Self::match_token_sequence(self, '=', Some('=')),
                '<' => Self::match_token_sequence(self, '<', Some('=')),
                '>' => Self::match_token_sequence(self, '>', Some('=')),
                '/' => Self::match_token_sequence(self, '/', Some('/')),
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => Self::string(self),
                _ => {
                    if Self::is_digit(c.unwrap()) {
                        Self::number(self);
                    } else if Self::is_alpha(c.unwrap()) {
                        Self::identifier(self);
                    } else {
                        Scanner::error(&self.line, "Unexpected character.")
                    }
                }
            }
        }
    }
    pub fn advance(&mut self) -> Option<char> {
        self.current = self.current + 1;
        self.source.chars().nth(self.current - 1)
    }
    pub fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    pub fn identifier(&mut self) {
        while Self::is_alphanumeric(Self::peek(&self)) {
            Self::advance(self);
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

        Self::add_token(self, ttype, Some(LiteralType::Nil))
    }
    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    pub fn is_alphanumeric(peeked_c: char) -> bool {
        Self::is_alpha(peeked_c) || Self::is_digit(peeked_c)
    }
    pub fn match_token(&mut self, expected: Option<char>) -> bool {
        if Self::is_at_end(&self) {
            return false;
        } else if self.source.chars().nth(self.current) != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    pub fn match_token_sequence(&mut self, case: char, expected: Option<char>) {
        let match_sequence = Self::match_token(self, expected);

        if case == '!' {
            let ttype = if match_sequence {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            Self::add_token(self, ttype, Some(LiteralType::Nil))
        } else if case == '=' {
            let ttype = if match_sequence {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            Self::add_token(self, ttype, Some(LiteralType::Nil))
        } else if case == '<' {
            let ttype = if match_sequence {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            Self::add_token(self, ttype, Some(LiteralType::Nil))
        } else if case == '>' {
            let ttype = if match_sequence {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            Self::add_token(self, ttype, Some(LiteralType::Nil))
        } else if case == '/' {
            if match_sequence {
                while Self::peek(&self) != '\n' && Self::is_at_end(&self) {
                    let _ = Self::advance(self);
                }
            } else {
                Self::add_token(self, TokenType::Slash, Some(LiteralType::Nil));
            };
        }
    }
    pub fn add_token(&mut self, ttype: TokenType, literal: Option<LiteralType>) {
        let text = self.source.get(self.start..self.current);

        if text != None {
            self.tokens.push(Token {
                ttype,
                lexeme: text.unwrap().to_string(),
                literal,
                line: self.line,
            });
        }
    }
    pub fn peek(&self) -> char {
        if Self::is_at_end(&self) {
            return '\0';
        } else {
            return self.source.chars().nth(self.current).unwrap();
        }
    }
    pub fn number(&mut self) {
        while Self::is_digit(Self::peek(&self)) {
            Self::advance(self);
        }

        if Self::peek(&self) == '.' && Self::is_digit(Self::peek_next(&self)) {
            Self::advance(self);

            while Self::is_digit(Self::peek(&self)) {
                Self::advance(self);
            }
        }

        let float_number: f32 = f32::from_str(self.source.get(self.start..self.current).unwrap())
            .ok()
            .unwrap();

        Self::add_token(
            self,
            TokenType::Number,
            Some(LiteralType::F32(float_number)),
        );
    }
    pub fn string(&mut self) {
        while Self::peek(&self) != '"' && !Self::is_at_end(&self) {
            if Self::peek(&self) != '\n' {
                self.line += 1;
                Self::advance(self);
            }
        }
        if Self::is_at_end(&self) {
            Scanner::error(&self.line, "Unterminated string");
        }

        Self::advance(self);

        let value: String = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        Self::add_token(self, TokenType::String, Some(LiteralType::String(value)))
    }
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
