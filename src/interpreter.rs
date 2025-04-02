use std::ops::Neg;

use crate::{
    expr::ExpressionType,
    token_type::{LiteralType, Token, TokenType},
};

pub struct Interpreter();

struct RuntimeError<'a> {
    token: Token,
    message: &'a str,
}

impl Interpreter {
    fn interpret(expr: ExpressionType) -> () {
        match expr {
            ExpressionType::BinaryExpr(expr) => {
                let value = Self::visit_binary_expr(expr);
                match value {
                    Ok(value) => {
                        println!("{}", Self::stringify(value));
                    }
                    Err(error) => {}
                }
            }
        }
    }
    fn stringify(value: LiteralType) -> String {
        match value {
            LiteralType::F32(f32_value) => {
                let mut text = f32_value.to_string();
                if text.ends_with(".0") {
                    let decimal_offset = text.find(".0").unwrap_or(text.len());
                    text = text.drain(..decimal_offset).collect();
                }
                text
            }
            LiteralType::Nil => "nil".to_string(),
            LiteralType::Bool(bool_value) => bool_value.to_string(),
            LiteralType::String(string_value) => string_value,
        }
    }
    fn visit_literal_expr(expr: ExpressionType) -> LiteralType {
        if let ExpressionType::LiteralExpr(literal) = expr {
            return literal.value;
        } else {
            panic!("visit_literal_expr must accept only literal as param");
        }
    }
    fn visit_grouping_expr(expr: ExpressionType) -> ExpressionType {
        if let ExpressionType::GroupingExpr(grouping) = expr {
            return *grouping.expression;
        } else {
            panic!("visit_grouping_expr must accept only grouping as param");
        }
    }
    fn visit_unary_expr(expr: ExpressionType) -> Result<LiteralType, RuntimeError<'static>> {
        if let ExpressionType::UnaryExpr(unary) = expr {
            let right = *unary.right;

            match unary.operator.ttype {
                TokenType::Minus => {
                    if let ExpressionType::LiteralExpr(literal) = right {
                        if let LiteralType::F32(f32_value) = literal.value {
                            return Ok(LiteralType::F32(f32_value.neg()));
                        } else {
                            return Err(RuntimeError {
                                message: "Operand must be a number",
                                token: unary.operator,
                            });
                        }
                    }
                }
                TokenType::Bang => {
                    return Ok(LiteralType::Bool(!Self::is_truthy(&right)));
                }
                _ => {}
            }
            return Ok(LiteralType::Nil);
        } else {
            panic!("visit_unary_expr must accept only unary as param");
        }
    }
    fn visit_binary_expr(expr: ExpressionType) -> Result<LiteralType, RuntimeError<'static>> {
        if let ExpressionType::BinaryExpr(binary) = expr {
            let left = *binary.left;
            let right = *binary.right;
            let mut left_literal_value = LiteralType::Nil;
            let mut right_literal_value = LiteralType::Nil;

            if let ExpressionType::LiteralExpr(left_literal) = left {
                if let ExpressionType::LiteralExpr(right_literal) = right {
                    if let LiteralType::F32(f32_left_value) = left_literal.value {
                        if let LiteralType::F32(f32_right_value) = right_literal.value {
                            left_literal_value = LiteralType::F32(f32_left_value);
                            right_literal_value = LiteralType::F32(f32_right_value);
                        }
                    }
                    if let LiteralType::String(string_left_value) = left_literal.value {
                        if let LiteralType::String(string_right_value) = right_literal.value {
                            left_literal_value = LiteralType::String(string_left_value);
                            right_literal_value = LiteralType::String(string_right_value);
                        }
                    }
                }
            }

            if let TokenType::Minus = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::F32(f32_left - f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::Plus = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::F32(f32_left + f32_right));
                    }
                    (LiteralType::String(string_left), LiteralType::String(string_right)) => {
                        return Ok(LiteralType::String(format!(
                            "{}{}",
                            string_left, string_right
                        )));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be two numbers or two strings",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::Slash = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::F32(f32_left / f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::Star = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::F32(f32_left * f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::Greater = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left > f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::GreaterEqual = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left >= f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::Less = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left < f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::LessEqual = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left <= f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::BangEqual = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left != f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else if let TokenType::EqualEqual = binary.operator.ttype {
                match (left_literal_value, right_literal_value) {
                    (LiteralType::F32(f32_left), LiteralType::F32(f32_right)) => {
                        return Ok(LiteralType::Bool(f32_left == f32_right));
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "Operands must be a number",
                            token: binary.operator,
                        })
                    }
                }
            } else {
                return Err(RuntimeError {
                    message: "Invalid operator",
                    token: binary.operator,
                });
            }
        } else {
            panic!("visit_binary_expr must accept only binary as param");
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
