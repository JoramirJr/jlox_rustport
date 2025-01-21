use std::str::FromStr;

use crate::token_type::LiteralType;
use crate::token_type::Token;
use crate::token_type::TokenType;
use crate::Main;

pub struct Scanner<LiteralType> {
    pub source: String,
    pub tokens: Option<Vec<Token<LiteralType>>>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner<LiteralType> {
    pub fn scan_tokens(mut self) -> Vec<Token<LiteralType>> {
        while !Self::is_at_end(&self) {
            self.start = self.current;
            Self::scan_token(&mut self);
        }

        self.tokens.as_mut().unwrap().push(Token {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
        self.tokens.unwrap()
    }
    fn scan_token(&mut self) {
        let c: Option<char> = Self::advance(self);

        let mut call_add_token = |ttype: TokenType| {
            Self::add_token(self, ttype, None);
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
                    if Self::is_digit(c.unwrap()) {
                        Self::number(self);
                    } else if Self::is_alpha(c.unwrap()) {
                        Self::identifier(self);
                    } else {
                        Main::error(&self.line, "Unexpected character.")
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

        let text: &str = self.source.get(self.start..self.current).unwrap();

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
            _ => None,
        };

        match ttype {
            Some(text) => Self::add_token(self, text, None),
            None => Self::add_token(self, TokenType::Identifier, None),
        }
    }
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_alphanumeric(peeked_c: char) -> bool {
        Self::is_alpha(peeked_c) || Self::is_digit(peeked_c)
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
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            Self::add_token(self, ttype, None)
        } else if case == '=' {
            let ttype = if match_sequence {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            Self::add_token(self, ttype, None)
        } else if case == '<' {
            let ttype = if match_sequence {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            Self::add_token(self, ttype, None)
        } else if case == '>' {
            let ttype = if match_sequence {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            Self::add_token(self, ttype, None)
        } else if case == '/' {
            if match_sequence {
                while Self::peek(&self) != '\n' && Self::is_at_end(&self) {
                    let _ = Self::advance(self);
                }
            } else {
                Self::add_token(self, TokenType::Slash, None);
            };
        }
    }
    fn add_token(&mut self, ttype: TokenType, literal: Option<LiteralType>) {
        //not sure if 'get' will bring me the intended substring
        let text = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();

        match self.tokens {
            Some(_) => {
                self.tokens.as_mut().unwrap().push(Token {
                    ttype,
                    lexeme: text,
                    literal,
                    line: self.line,
                });
            }
            None => {
                let _ = self.tokens.insert(Vec::from([Token {
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

        let float_number: f32 = f32::from_str(self.source.get(self.start..self.current).unwrap())
            .ok()
            .unwrap();

        Self::add_token(
            self,
            TokenType::Number,
            Some(LiteralType::F32(float_number)),
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
            Main::error(&self.line, "Unterminated string");
        }

        Self::advance(self);

        let value: String = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        Self::add_token(self, TokenType::String, Some(LiteralType::String(value)))
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
