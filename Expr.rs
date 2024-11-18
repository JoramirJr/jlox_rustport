use crate::token_type::Token;

mod Expr {
    struct Binary {
        left: &str,
        operator: Token,
        right: &str,
    }
    struct Grouping {
        expression: &str,
    }
    struct Literal<T> {
        value: Option<T>,
    }
    struct Unary {
        operator: Token,
        right: &str,
    }
}
