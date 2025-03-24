pub mod ast_printer;
pub mod expr;
pub mod parser;
pub mod scanner;
pub mod token_type;
pub mod lox; 

use expr::{Binary, Grouping, Literal, Unary};

pub enum Visitor {
    VisitBinary(Binary),
    VisitGrouping(Grouping),
    VisitLiteral(Literal),
    VisitUnary(Unary),
}
