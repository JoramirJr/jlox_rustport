use crate::token_type::{LiteralType, Token};

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Variable(Variable),
    Logical(Logical),
    Unary(Unary),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<ExpressionType>,
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<ExpressionType>,
    pub operator: Token,
    pub right: Box<ExpressionType>,
}
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<ExpressionType>,
}
#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralType,
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}
#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<ExpressionType>,
    pub operator: Token,
    pub right: Box<ExpressionType>,
}
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExpressionType>,
}
