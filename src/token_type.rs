use crate::{interpreter::Interpreter, Callable};

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

#[derive(Debug, Clone)]
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
    pub literal: LiteralType,
    pub line: u32,
}

impl Callable for LiteralType {
    fn call(
        &self,
        interpreter: Option<&mut Interpreter>,
        arguments: Vec<LiteralType>,
    ) -> LiteralType {
        todo!()
    }
    fn arity(&self) -> usize {
        todo!()
    }
}
