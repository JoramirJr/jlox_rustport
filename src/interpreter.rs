use std::ops::Neg;

use crate::{
    expr::{Binary, ExpressionType, Grouping, Literal, Unary},
    lox::Lox,
    stmt::StmtType,
    token_type::{LiteralType, Token, TokenType},
};

pub struct Interpreter();

// pub static INTERPRETER_SINGLETON: LazyLock<Mutex<Interpreter>> =
//     LazyLock::new(|| Mutex::new(Interpreter {}));

#[derive(Debug)]
pub struct RuntimeError<'a> {
    pub token: Token,
    pub message: &'a str,
}

impl Interpreter {
    pub fn interpret(statements: Vec<StmtType>) -> () {
        for statement in statements {
            let execute_result = Self::execute(statement);

            if let Err(runtime_error) = execute_result {
                Lox::runtime_error(runtime_error);
            }
        }
    }
    pub fn evaluate(expr: ExpressionType) -> Result<LiteralType, RuntimeError<'static>> {
        match expr {
            ExpressionType::BinaryExpr(binary) => Self::visit_binary_expr(binary),
            ExpressionType::GroupingExpr(grouping) => Self::visit_grouping_expr(grouping),
            ExpressionType::LiteralExpr(literal) => Self::visit_literal_expr(literal),
            ExpressionType::UnaryExpr(unary) => Self::visit_unary_expr(unary),
            ExpressionType::VariableExpr(variable) => todo!(),
        }
    }
    fn execute(stmt: StmtType) -> Result<LiteralType, RuntimeError<'static>> {
        match stmt {
            StmtType::ExpressionExpr(expr) => Self::visit_expression_stmt(expr.expression),
            StmtType::PrintExpr(print) => Self::visit_print_stmt(print.expression),
            StmtType::VarExpr(var) => todo!(),
        }
    }
    fn visit_expression_stmt(expr: ExpressionType) -> Result<LiteralType, RuntimeError<'static>> {
        Self::evaluate(expr)
    }
    fn visit_print_stmt(expr: ExpressionType) -> Result<LiteralType, RuntimeError<'static>> {
        let value = Self::evaluate(expr);

        match value {
            Ok(literal) => {
                println!("{:?}", Self::stringify(&literal));
                Ok(literal)
            }
            Err(error) => Err(error),
        }
    }
    pub fn stringify(value: &LiteralType) -> String {
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
            LiteralType::String(string_value) => string_value.clone(),
        }
    }
    pub fn visit_literal_expr(literal: Literal) -> Result<LiteralType, RuntimeError<'static>> {
        Ok(literal.value)
    }
    pub fn visit_grouping_expr(grouping: Grouping) -> Result<LiteralType, RuntimeError<'static>> {
        Self::evaluate(*grouping.expression)
    }
    pub fn visit_unary_expr(unary: Unary) -> Result<LiteralType, RuntimeError<'static>> {
        let right_r_value = Self::evaluate(*unary.right);

        if let Err(right_operand_error) = right_r_value {
            return Err(RuntimeError {
                message: right_operand_error.message,
                token: right_operand_error.token,
            });
        }

        let right_value = right_r_value.unwrap();

        match unary.operator.ttype {
            TokenType::Minus => {
                if let LiteralType::F32(f32_value) = right_value {
                    return Ok(LiteralType::F32(f32_value.neg()));
                } else {
                    return Err(RuntimeError {
                        message: "Operand must be a number",
                        token: unary.operator,
                    });
                }
            }
            TokenType::Bang => {
                return Ok(LiteralType::Bool(!Self::is_truthy(right_value)));
            }
            _ => {}
        }
        Ok(LiteralType::Nil)
    }
    pub fn visit_binary_expr(binary: Binary) -> Result<LiteralType, RuntimeError<'static>> {
        let left = *binary.left;
        let right = *binary.right;
        let left_r_value = Self::evaluate(left);
        let right_r_value = Self::evaluate(right);

        if let Err(left_operand_error) = left_r_value {
            return Err(RuntimeError {
                message: left_operand_error.message,
                token: left_operand_error.token,
            });
        }

        if let Err(right_operand_error) = right_r_value {
            return Err(RuntimeError {
                message: right_operand_error.message,
                token: right_operand_error.token,
            });
        }

        let left_value = left_r_value.unwrap();
        let right_value = right_r_value.unwrap();

        if let TokenType::Minus = binary.operator.ttype {
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
            match (left_value, right_value) {
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
    }
    pub fn is_truthy(item: LiteralType) -> bool {
        match item {
            LiteralType::Bool(bool) => {
                return bool;
            }
            LiteralType::Nil => {
                return false;
            }
            _ => {
                return true;
            }
        }
    }
}
