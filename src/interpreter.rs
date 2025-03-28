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
    fn visitBinaryExpr(expr: ExpressionType) -> Option<LiteralType> {
        if let ExpressionType::BinaryExpr(binary) = expr {
            let left = *binary.left;
            let right = *binary.right;
            let mut left_literal_value;
            let mut right_literal_value;

            if let ExpressionType::LiteralExpr(left_literal) = left {
                if let ExpressionType::LiteralExpr(right_literal) = right {
                    if let LiteralType::F32(f32_left_value) = left_literal.value {
                        if let LiteralType::F32(f32_right_value) = right_literal.value {
                            left_literal_value = f32_left_value;
                            right_literal_value = f32_right_value;
                        }
                    }
                    if let LiteralType::String(string_left_value) = left_literal.value {
                        if let LiteralType::String(string_right_value) = right_literal.value {
                            left_literal_value = string_left_value;
                            right_literal_value = string_right_value;
                        }
                    }
                }
            }

            if let TokenType::Minus = binary.operator.ttype {
                    return Some(LiteralType::F32(left_literal_value - right_literal_value));
            } else if let TokenType::Plus = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) {
                    return Some(LiteralType::F32(f32_left + f32_right));
                },
                (LiteralType::String(string_left), LiteralType::String(string_right)) {
                    return Some(LiteralType::String(concat!(string_left, string_right).to_string()));
                },  
                }
                return Some(LiteralType::F32(left_literal_value / right_literal_value));
            } 
            else if let TokenType::Slash = binary.operator.ttype {
                return Some(LiteralType::F32(left_literal_value / right_literal_value));
            } else if let TokenType::Star = binary.operator.ttype {
                return Some(LiteralType::F32(left_literal_value * right_literal_value));
            } else {
                return None;
            }
            }
        }
    }
    fn is_truthy(item: &ExpressionType) -> bool {
        match item {
            ExpressionType::LiteralExpr(literal) => match literal.value {
                LiteralType::Bool(bool_literal) => {
                    return bool_literal;
                }
                LiteralType::Nil => {
                    return false;
                }
                _ => {
                    return true;
                }
            },
            _ => {
                return true;
            }
        }
    }
}
