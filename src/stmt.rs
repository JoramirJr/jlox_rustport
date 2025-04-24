use crate::{expr::ExpressionType, token_type::Token};

#[derive(Debug)]
pub enum StmtType {
    ExpressionExpr(Expression),
    IfExpr(If),
    BlockExpr(Block),
    VarExpr(Var),
    PrintExpr(Print),
}

#[derive(Debug)]
pub struct Expression {
    pub expression: ExpressionType,
}
#[derive(Debug)]
pub struct If {
    pub condition: ExpressionType,
    pub then_branch: Box<StmtType>,
    pub else_branch: Box<StmtType>,
}
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<StmtType>,
}
#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<ExpressionType>,
}
#[derive(Debug)]
pub struct Print {
    pub expression: ExpressionType,
}
