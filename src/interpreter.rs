use std::{
    collections::HashMap,
    ops::Neg,
    sync::{LazyLock, Mutex, MutexGuard},
};

use crate::{
    environment::Environment,
    expr::{Assign, Binary, ExpressionType, Grouping, Literal, Unary, Variable},
    lox::Lox,
    stmt::{Block, StmtType, Var},
    token_type::{LiteralType, Token, TokenType},
};

pub struct Interpreter {
    environment: Environment,
}

pub static INTERPRETER_SINGLETON: LazyLock<Mutex<Interpreter>> = LazyLock::new(|| {
    Mutex::new(Interpreter {
        environment: Environment {
            values: HashMap::new(),
            enclosing: None,
        },
    })
});

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

type DefaultResult = Result<LiteralType, RuntimeError>;

impl Interpreter {
    pub fn interpret(statements: Vec<StmtType>) -> () {
        let interpreter_singleton = INTERPRETER_SINGLETON.lock();

        match interpreter_singleton {
            Ok(mut interpreter) => {
                for statement in statements {
                    let execute_result = Self::execute(statement, &mut interpreter);

                    if let Err(runtime_error) = execute_result {
                        Lox::runtime_error(runtime_error);
                    }
                }
            }
            Err(err) => {
                panic!("Interpreter singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn evaluate(expr: ExpressionType) -> DefaultResult {
        match expr {
            ExpressionType::BinaryExpr(binary) => Self::visit_binary_expr(binary),
            ExpressionType::GroupingExpr(grouping) => Self::visit_grouping_expr(grouping),
            ExpressionType::LiteralExpr(literal) => Self::visit_literal_expr(literal),
            ExpressionType::UnaryExpr(unary) => Self::visit_unary_expr(unary),
            ExpressionType::VariableExpr(variable) => Self::visit_variable_expr(variable),
            ExpressionType::AssignExpr(assignment) => Self::visit_assign_expr(assignment),
        }
    }
    fn execute(stmt: StmtType, interpreter: &mut MutexGuard<'_, Interpreter>) -> DefaultResult {
        match stmt {
            StmtType::ExpressionExpr(expr) => Self::visit_expression_stmt(expr.expression),
            StmtType::PrintExpr(print) => Self::visit_print_stmt(print.expression),
            StmtType::VarExpr(var) => Self::visit_var_stmt(var),
            StmtType::BlockExpr(block) => Self::visit_block_stmt(block, interpreter),
        }
    }
    fn visit_block_stmt(
        stmt: Block,
        interpreter: &mut MutexGuard<'_, Interpreter>,
    ) -> DefaultResult {
        Self::execute_block(
            stmt.statements,
            Environment {
                enclosing: Some(Box::new(interpreter.environment.clone())),
                values: HashMap::new(),
            },
            interpreter,
        )
    }
    fn execute_block(
        statements: Vec<StmtType>,
        environment: Environment,
        interpreter: &mut MutexGuard<'_, Interpreter>,
    ) -> DefaultResult {
        let previous: Environment = interpreter.environment.clone();
        interpreter.environment = environment;
        let mut curr_execute_result: LiteralType = LiteralType::Nil;

        for statement in statements {
            let execute_result = Self::execute(statement, interpreter);

            match execute_result {
                Ok(literal_type) => {
                    curr_execute_result = literal_type;
                }
                Err(err) => {
                    interpreter.environment = previous;
                    return Err(err);
                }
            };
        }
        Ok(curr_execute_result)
    }
    fn visit_expression_stmt(expr: ExpressionType) -> DefaultResult {
        Self::evaluate(expr)
    }
    fn visit_print_stmt(expr: ExpressionType) -> DefaultResult {
        let value = Self::evaluate(expr);

        match value {
            Ok(literal) => {
                println!("{:?}", Self::stringify(&literal));
                Ok(literal)
            }
            Err(error) => Err(error),
        }
    }
    fn visit_var_stmt(stmt: Var) -> DefaultResult {
        let interpreter_singleton = INTERPRETER_SINGLETON.lock();

        let mut value: LiteralType = LiteralType::Nil;

        match interpreter_singleton {
            Ok(mut interpreter) => {
                if let Some(expr_initializer) = stmt.initializer {
                    let literal = Ok(Self::evaluate(expr_initializer))?;
                    value = literal.unwrap();
                }
                interpreter
                    .environment
                    .define(stmt.name.lexeme, value.clone());

                std::mem::drop(interpreter);
                return Ok(value);
            }
            Err(err) => {
                panic!("Interpreter singleton lock unwrap failed; error: {:?}", err);
            }
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
    pub fn visit_literal_expr(literal: Literal) -> DefaultResult {
        Ok(literal.value)
    }
    pub fn visit_grouping_expr(grouping: Grouping) -> DefaultResult {
        Self::evaluate(*grouping.expression)
    }
    pub fn visit_unary_expr(unary: Unary) -> DefaultResult {
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
                        message: String::from("Operand must be a number"),
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
    pub fn visit_variable_expr(expr: Variable) -> DefaultResult {
        let interpreter_singleton = INTERPRETER_SINGLETON.lock();

        match interpreter_singleton {
            Ok(interpreter) => {
                let get_result = interpreter.environment.get(&expr.name);
                std::mem::drop(interpreter);
                return get_result;
            }
            Err(err) => {
                panic!("Interpreter singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn visit_assign_expr(expr: Assign) -> DefaultResult {
        let interpreter_singleton = INTERPRETER_SINGLETON.lock();

        match interpreter_singleton {
            Ok(mut interpreter) => {
                let value = Self::evaluate(*expr.value)?;

                let get_result = interpreter.environment.assign(expr.name, value);
                std::mem::drop(interpreter);
                return get_result;
            }
            Err(err) => {
                panic!("Interpreter singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    pub fn visit_binary_expr(binary: Binary) -> DefaultResult {
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be two numbers or two strings"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
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
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else {
            return Err(RuntimeError {
                message: String::from("Invalid operator"),
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
