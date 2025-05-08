use std::io;
use std::io::Write;
use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::Mutex;

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

pub static SCANNER_SINGLETON: LazyLock<Mutex<Scanner>> = LazyLock::new(|| {
    Mutex::new(Scanner {
        source: String::new(),
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    })
});

impl Scanner {
    pub fn scan_tokens(source_file: String) -> Vec<Token> {
        let scanner_singleton = SCANNER_SINGLETON.lock();

        match scanner_singleton {
            Ok(mut scanner) => {
                scanner.source = source_file;

                while !Self::is_at_end(&scanner) {
                    scanner.start = scanner.current;
                    Self::scan_token(&mut scanner);
                }

                let line = scanner.line;

                scanner.tokens.push(Token {
                    ttype: TokenType::Eof,
                    lexeme: String::new(),
                    literal: LiteralType::Nil,
                    line: line,
                });
                let tokens = scanner.tokens.clone();
                std::mem::drop(scanner);
                tokens
            }
            Err(err) => {
                panic!("Scanner singleton lock unwrap failed; error: {:?}", err);
            }
        }
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

        let mut call_add_token = |ttype: TokenType| {
            Self::add_token(self, ttype, LiteralType::Nil);
        };

        if c != None {
            match c.unwrap() {
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
        //+1 and -1 implemented to not jump the first chars position
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

        let text: Option<&str> = self.source.get(self.start..self.current);

        let ttype = match text {
            Some("and") => TokenType::And,
            Some("class") => TokenType::Class,
            Some("else") => TokenType::Else,
            Some("false") => TokenType::False,
            Some("for") => TokenType::For,
            Some("fun") => TokenType::Fun,
            Some("if") => TokenType::If,
            Some("nil") => TokenType::Nil,
            Some("or") => TokenType::Or,
            Some("print") => TokenType::Print,
            Some("return") => TokenType::Return,
            Some("super") => TokenType::Super,
            Some("this") => TokenType::This,
            Some("true") => TokenType::True,
            Some("var") => TokenType::Var,
            Some("while") => TokenType::While,
            _ => TokenType::Nil,
        };

        match ttype {
            TokenType::Nil => Self::add_token(self, TokenType::Identifier, LiteralType::Nil),
            _ => Self::add_token(self, ttype, LiteralType::Nil),
        }
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
    pub fn add_token(&mut self, ttype: TokenType, literal: LiteralType) {
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

        Self::add_token(self, TokenType::Number, LiteralType::F32(float_number));
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
        Self::add_token(self, TokenType::String, LiteralType::String(value))
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
