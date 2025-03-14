pub mod expr;

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