// use std::fmt::Debug;

// #[derive(Debug)]
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

pub struct Token<T> {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<T>,
    pub line: u32,
}

// impl<'a, T: Debug> Token<T> {
//     pub fn to_string(&self) -> String {
//         format!("{:?} {} {:?}", &self.ttype, &self.lexeme, &self.literal)
//     }
// }
