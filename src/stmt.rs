use crate::{expr::ExpressionType, token_type::Token};

#[derive(Debug, Clone)]
pub enum StmtType {
    Expression(Expression),
    If(If),
    Block(Block),
    Var(Var),
    Print(Print),
    While(While),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: ExpressionType,
}
#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<ExpressionType>,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtType>,
}
#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<ExpressionType>,
}
#[derive(Debug, Clone)]
pub struct Print {
    pub expression: ExpressionType,
}
#[derive(Debug, Clone)]
pub struct While {
    pub condition: ExpressionType,
    pub body: Box<StmtType>,
}
