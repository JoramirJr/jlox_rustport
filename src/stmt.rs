use crate::token_type::{LiteralType, Token};

#[derive(Debug)]
pub enum ExpressionType {
    ExpressionExpr(Expression),
    PrintExpr(Print),
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Box<ExpressionType>,
}
#[derive(Debug)]
pub struct Print {
    pub expression: Box<ExpressionType>,
}
