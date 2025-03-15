pub mod expr;
pub mod token_type;
pub mod ast_printer;
pub mod parser;
pub mod scanner;

use expr::{Binary, Grouping, Literal, Unary};

pub enum Visitor {
    VisitBinary(Binary),
    VisitGrouping(Grouping),
    VisitLiteral(Literal),
    VisitUnary(Unary),
}

pub trait ScanningParsingCommon {
    fn error(line: &u32, message: &str) -> ();
    fn report(line: &u32, location: String, message: &str);
}