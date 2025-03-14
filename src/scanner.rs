use std::io;
use std::io::Write;
use std::str::FromStr;

use jlox_rustport::ScanningParsingCommon;

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

impl ScanningParsingCommon for Scanner {
    fn error(line: &u32, message: &str) {
        Self::report(line, String::new(), message);
    }
    fn report(line: &u32, location: String, message: &str) {
        let err_msg = format!("[line {}] Error {}: {}", line, location, message);
        let mut err_out_handler = io::stderr();
        let _ = err_out_handler.write_all(err_msg.as_bytes());
    }
}

impl Scanner {
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !Self::is_at_end(&self) {
            self.start = self.current;
            Self::scan_token(&mut self);
        }

        self.tokens.as_mut().push(Token {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: LiteralType::Nil,
            line: self.line,
        });
        self.tokens
    }
    fn scan_token(&mut self) {
        let c: Option<char> = Self::advance(self);

        let mut call_add_token = |ttype: TokenType| {
            Self::add_token(self, ttype, LiteralType::Nil);
        };

        if c != None {
            match Some(c) {
                '(' => call_add_token(TokenType::LeftParen),
                ')' => call_add_token(TokenType::RightParen),
                '{' => call_add_token(TokenType::LeftBrace),
                '}' => call_add_token(TokenType::RightBrace),
                ',' => call_add_token(TokenType::Comma),
                '.' => call_add_token(TokenType::Dot),
                '-' => call_add_token(TokenType::Minus),
                '+' => call_add_token(TokenType::Plus),
                ';' => call_add_token(TokenType::Semicolon),
                '*' => call_add_token(TokenType::Star),
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
                    if Self::is_digit(c) {
                        Self::number(self);
                    } else if Self::is_alpha(c) {
                        Self::identifier(self);
                    } else {
                        Scanner::error(&self.line, "Unexpected character.")
                    }
                }
            }
        }
    }
    fn advance(&mut self) -> Option<char> {
        //+1 and -1 implemented to not jump the first chars position
        self.current = self.current + 1;
        self.source.chars().nth(self.current - 1)
    }
    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    fn identifier(&mut self) {
        while Self::is_alphanumeric(Self::peek(&self)) {
            Self::advance(self);
        }

        let text: &str = self.source.get(self.start..self.current);

        let ttype = match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => LiteralType::Nil,
        };

        match ttype {
            Some(text) => Self::add_token(self, text, LiteralType::Nil),
            LiteralType::Nil => Self::add_token(self, TokenType::Identifier, LiteralType::Nil),
        }
    }
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_alphanumeric(peeked_c: char) -> bool {
        Self::is_alpha(peeked_c) || Self::is_digit(peeked_c)
    }
    fn match_token(&mut self, expected: Option<char>) -> bool {
        if Self::is_at_end(&self) {
            return false;
        } else if self.source.chars().nth(self.current) != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    fn match_token_sequence(&mut self, case: char, expected: Option<char>) {
        let match_sequence = Self::match_token(self, expected);

        if case == '!' {
            let ttype = if match_sequence {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            Self::add_token(self, ttype, LiteralType::Nil)
        } else if case == '=' {
            let ttype = if match_sequence {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            Self::add_token(self, ttype, LiteralType::Nil)
        } else if case == '<' {
            let ttype = if match_sequence {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            Self::add_token(self, ttype, LiteralType::Nil)
        } else if case == '>' {
            let ttype = if match_sequence {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            Self::add_token(self, ttype, LiteralType::Nil)
        } else if case == '/' {
            if match_sequence {
                while Self::peek(&self) != '\n' && Self::is_at_end(&self) {
                    let _ = Self::advance(self);
                }
            } else {
                Self::add_token(self, TokenType::Slash, LiteralType::Nil);
            };
        }
    }
    fn add_token(&mut self, ttype: TokenType, literal: LiteralType) {
        //not sure if 'get' will bring me the intended substring
        let text = self
            .source
            .get(self.start..self.current)
            
            .to_string();

        match self.tokens {
            Some(_) => {
                self.tokens.as_mut().push(Token {
                    ttype,
                    lexeme: text,
                    literal,
                    line: self.line,
                });
            }
            LiteralType::Nil => {
                let _ = self.tokens.insert(self, Vec::from([Token {
                    ttype,
                    lexeme: text,
                    literal,
                    line: self.line,
                }]));
            }
        }
    }
    fn peek(&self) -> char {
        if Self::is_at_end(&self) {
            '\0'
        } else {
            self.source.chars().nth(self.current)
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

        let float_number: f32 = f32::from_str(self.source.get(self.start..self.current))
            .ok()
            ;

        Self::add_token(
            self,
            TokenType::Number,
            LiteralType::F32(float_number),
        );
    }
    fn string(&mut self) {
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
            
            .to_string();
        Self::add_token(self, TokenType::String, LiteralType::String(value))
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
