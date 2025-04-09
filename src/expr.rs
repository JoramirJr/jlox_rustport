use crate::token_type::{LiteralType, Token};

#[derive(Debug)]
pub enum ExpressionType {
    BinaryExpr(Binary),
    GroupingExpr(Grouping),
    LiteralExpr(Literal),
    VariableExpr(Variable),
    UnaryExpr(Unary),
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<ExpressionType>,
    pub operator: Token,
    pub right: Box<ExpressionType>,
}
#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<ExpressionType>,
}
#[derive(Debug)]
pub struct Literal {
    pub value: LiteralType,
}
#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}
#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExpressionType>,
}
