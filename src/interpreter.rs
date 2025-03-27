use crate::{expr::{ExpressionType, Unary}, token_type::{LiteralType, TokenType}};

pub struct Interpreter();

impl Interpreter {
    fn visitLiteralExpr(expr: ExpressionType) -> LiteralType {
        if let ExpressionType::LiteralExpr(literal) = expr {
            return literal.value;
        } else {
            panic!("visitLiteralExpr must accept only literal as param");
        }
    }
    fn visitGroupingExpr(expr: ExpressionType) -> ExpressionType {
        if let ExpressionType::GroupingExpr(grouping) = expr {
            return *grouping.expression;
        } else {
            panic!("visitGroupingExpr must accept only grouping as param");
        }
    }
    fn visitUnaryExpr(expr: ExpressionType) -> ExpressionType {
        if let ExpressionType::UnaryExpr(unary) = expr {
            let right = *unary.right;

            match unary.operator.ttype {
                TokenType::Minus => {
                    return -right;
                }
            }
        } else {
            panic!("visitUnaryExpr must accept only unary as param");
        }
    }
}
