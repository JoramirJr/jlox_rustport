use std::ops::Neg;

use crate::{
    expr::ExpressionType,
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
    fn visitUnaryExpr(expr: ExpressionType) -> Option<LiteralType> {
        if let ExpressionType::UnaryExpr(unary) = expr {
            let right = *unary.right;

            match unary.operator.ttype {
                TokenType::Minus => {
                    if let ExpressionType::LiteralExpr(literal) = right {
                        if let LiteralType::F32(f32_value) = literal.value {
                            return Some(LiteralType::F32(f32_value.neg()));
                        }
                    }
                }
                TokenType::Bang => {
                    return Some(LiteralType::Bool(!Self::is_truthy(&right)));
                }
                _ => {}
            }
            return Some(LiteralType::Nil);
        } else {
            return None;
        }
    }
    fn is_truthy(item: &ExpressionType) -> bool {
        match item {
            ExpressionType::LiteralExpr(literal) => {
                match literal.value {
                LiteralType::Bool(bool_literal) => {
                    return bool_literal;
                }
                LiteralType::Nil => {
                    return false;
                }
                _ => {
                    return true;
                }
            }
            },
            _ => {
                return true;
            }
        }
    }
}
