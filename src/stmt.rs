#[derive(Debug)]
pub enum StmtType {
    ExpressionExpr(Expression),
    PrintExpr(Print),
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Box<StmtType>,
}
#[derive(Debug)]
pub struct Print {
    pub expression: Box<StmtType>,
}
