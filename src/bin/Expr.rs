pub mod Expr {
    struct Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    }
    struct Grouping {
        expression: Expr,
    }
    struct Literal {
        value: Struct,
    }
    struct Unary {
        operator: Token,
        right: Expr,
    }
}
