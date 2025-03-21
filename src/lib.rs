pub mod ast_printer;
pub mod expr;
pub mod parser;
pub mod scanner;
pub mod token_type;

use expr::{Binary, Grouping, Literal, Unary};

pub enum Visitor {
    VisitBinary(Binary),
    VisitGrouping(Grouping),
    VisitLiteral(Literal),
    VisitUnary(Unary),
}

pub trait ScanningParsingCommon {
    fn error(line: &u32, message: &str) -> String;
    fn report(line: &u32, location: String, message: &str) -> String;
}
