use crate::token_type::Token;

mod Expr {
    struct Binary {
        left: String,
        operator: Token,
        right: String,
    }
    struct Grouping {
        expression: String,
    }
    struct Literal<T> {
        value: Option<T>,
    }
    struct Unary {
        operator: Token,
        right: String,
    }
}
