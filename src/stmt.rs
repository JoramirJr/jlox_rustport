use crate::expr::ExpressionType;

#[derive(Debug)]
pub enum StmtType {
    ExpressionExpr(Expression),
    PrintExpr(Print),
}

#[derive(Debug)]
pub struct Expression {
    pub expression: ExpressionType,
}
#[derive(Debug)]
pub struct Print {
    pub expression: ExpressionType,
}
