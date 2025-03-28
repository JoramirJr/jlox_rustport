use std::ops::Neg;

use crate::{
    expr::{ExpressionType, Unary},
    token_type::{LiteralType, TokenType},
};

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
    fn visitUnaryExpr(expr: ExpressionType) -> LiteralType {
        if let ExpressionType::UnaryExpr(unary) = expr {
            let right = *unary.right;

            match unary.operator.ttype {
                TokenType::Minus => {
                    if let ExpressionType::LiteralExpr(literal) = right {
                        if let LiteralType::F32(f32_value) = literal.value {
                            return LiteralType::F32(f32_value.neg());
                        }
                    }
                }
                TokenType::Bang => { 
                    return !Self::is_truthy(&right);
                 }
            }
            return LiteralType::Nil;
        } else {
            panic!("visitUnaryExpr must accept only unary as param");
        }
    }
    fn is_truthy(item: &LiteralType) -> bool {
        true
    }
}
