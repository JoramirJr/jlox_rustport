pub enum Visitor<T> {
    VisitBinary(Binary),
    VisitGrouping(Grouping),
    VisitLiteral(Literal<T>),
    VisitUnary(Unary),
}

pub trait ScanningParsingCommon {
    fn error(line: &u32, message: &str) -> ();
    fn report(line: &u32, location: String, message: &str);
}

pub mod token_type {
    // use std::fmt::Debug;

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
    }

    #[derive(Debug, Clone)]
    pub struct Token<LiteralType> {
        pub ttype: TokenType,
        pub lexeme: String,
        pub literal: Option<LiteralType>,
        pub line: u32,
    }
}
