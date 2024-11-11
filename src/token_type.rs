// use std::fmt::Debug;

#[derive(Debug)]
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

mod ExprExpr{

    structBinary{    left=Expr,    operator=Token,    right=Expr,}
    structGrouping{    expression=Expr,}
    structLiteral{    =Struct,}
    structUnary{    operator=Token,    right=Expr,
}

#[derive(Debug)]
pub enum LiteralType {
    String(String),
    F32(f32),
}

#[derive(Debug)]
pub struct Token<LiteralType> {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: u32,
}

// impl<'a, T: Debug> Token<T> {
//     pub fn to_string(&self) -> String {
//         format!("{:?} {} {:?}", &self.ttype, &self.lexeme, &self.literal)
//     }
// }
