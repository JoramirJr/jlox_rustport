#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    //one Or Two Character Tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //literals
    Identifier,
    String,
    Number,
    //keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    String(String),
    F32(f32),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: u32,
}

pub mod LoxCallable {
    use crate::{
        interpreter::{Interpreter, RuntimeError},
        token_type::{LiteralType, Token},
    };

    pub fn call_function(
        callee: &LiteralType,
        arguments: Vec<LiteralType>,
        interpreter: Option<&mut Interpreter>,
        expr_closing_paren: Token,
    ) -> Result<LiteralType, RuntimeError> {
        match callee {
            LiteralType::String(_) => {
                return Err(RuntimeError {
                    token: expr_closing_paren,
                    message: "Can only call functions and classes.".to_string(),
                })
            }
            LiteralType::F32(_) => {
                return Err(RuntimeError {
                    token: expr_closing_paren,
                    message: "Can only call functions and classes.".to_string(),
                })
            }
            LiteralType::Bool(_) => {
                return Err(RuntimeError {
                    token: expr_closing_paren,
                    message: "Can only call functions and classes.".to_string(),
                })
            }
            LiteralType::Nil => {
                return Err(RuntimeError {
                    token: expr_closing_paren,
                    message: "Can only call functions and classes.".to_string(),
                })
            }
        }
    }
    pub fn arity(&self) -> usize {
        todo!()
    }

    pub fn to_string(&self) -> String {
        todo!()
    }
}
