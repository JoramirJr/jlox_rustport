// use std::fmt::Debug;

// #[derive(Debug)]
pub enum TokenType {
    //single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    //One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    //Literals
    IDENTIFIER,
    STRING,
    NUMBER,
    //Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
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
