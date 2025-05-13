use std::{cell::RefCell, collections::HashMap, ops::Neg, rc::Rc};

use crate::{
    environment::Environment,
    expr::{Assign, Binary, ExpressionType, Grouping, Literal, Logical, Unary, Variable},
    lox::Lox,
    stmt::{Block, If, StmtType, Var, While},
    token_type::{LiteralType, Token, TokenType},
};
pub struct Interpreter<'a> {
    pub environment: Rc<RefCell<&'a mut Environment<'a>>>,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

type DefaultResult = Result<LiteralType, RuntimeError>;

impl<'a> Interpreter<'a> {
    pub fn interpret(&'a mut self, statements: Vec<StmtType>) -> () {
        for statement in statements {
            let execute_result = Self::execute(self, statement);

            if let Err(runtime_error) = execute_result {
                Lox::runtime_error(runtime_error);
            }
        }
    }
    pub fn evaluate(&mut self, expr: ExpressionType) -> DefaultResult {
        match expr {
            ExpressionType::Binary(binary) => Self::visit_binary_expr(self, binary),
            ExpressionType::Grouping(grouping) => Self::visit_grouping_expr(self, grouping),
            ExpressionType::Literal(literal) => Self::visit_literal_expr(literal),
            ExpressionType::Unary(unary) => Self::visit_unary_expr(self, unary),
            ExpressionType::Variable(variable) => Self::visit_variable_expr(self, variable),
            ExpressionType::Assign(assignment) => Self::visit_assign_expr(self, assignment),
            ExpressionType::Logical(logical) => Self::visit_logical_expr(self, logical),
        }
    }
    fn execute(&'a mut self, stmt: StmtType) -> DefaultResult {
        match stmt {
            StmtType::Expression(expr) => Self::visit_expression_stmt(self, expr.expression),
            StmtType::Print(print) => Self::visit_print_stmt(self, print.expression),
            StmtType::Var(var) => Self::visit_var_stmt(self, var),
            StmtType::Block(block) => Self::visit_block_stmt(self, block),
            StmtType::If(if_stmt) => Self::visit_if_stmt(self, if_stmt),
            StmtType::While(while_stmt) => Self::visit_while_stmt(self, while_stmt),
        }
    }
    fn visit_block_stmt(&'a mut self, stmt: Block) -> DefaultResult {
        Self::execute_block(self, stmt.statements)
    }
    fn execute_block(&'a mut self, statements: Vec<StmtType>) -> DefaultResult {
        let mut curr_env: Environment<'a> = Environment {
            enclosing: Some(self.environment),
            values: HashMap::new(),
        };
        self.environment = Rc::new(RefCell::new(&mut curr_env));
        let mut curr_execute_result: LiteralType = LiteralType::Nil;

        for statement in statements {
            let execute_result: Result<LiteralType, RuntimeError> = Self::execute(self, statement);

            match execute_result {
                Ok(literal_type) => {
                    curr_execute_result = literal_type;
                }
                Err(err) => {
                    self.environment = self.environment;
                    return Err(err);
                }
            };
        }
        self.environment = self.environment;
        Ok(curr_execute_result)
    }
    fn visit_expression_stmt(&mut self, expr: ExpressionType) -> DefaultResult {
        Self::evaluate(self, expr)
    }
    fn visit_if_stmt(&'a mut self, stmt: If) -> DefaultResult {
        let evaluate_result: LiteralType = Self::evaluate(self, *stmt.condition)?;
        if Self::is_truthy(&evaluate_result) {
            Self::execute(self, StmtType::Block(stmt.then_branch))
        } else if let Some(else_branch) = stmt.else_branch {
            Self::execute(self, StmtType::Block(else_branch))
        } else {
            Ok(LiteralType::Nil)
        }
    }
    fn visit_print_stmt(&mut self, expr: ExpressionType) -> DefaultResult {
        let value = Self::evaluate(self, expr);

        match value {
            Ok(literal) => {
                // println!("{:?}", Self::stringify(&literal));
                Ok(literal)
            }
            Err(error) => Err(error),
        }
    }
    fn visit_var_stmt(&mut self, stmt: Var) -> DefaultResult {
        let mut value: LiteralType = LiteralType::Nil;

        if let Some(expr_initializer) = stmt.initializer {
            let literal = Ok(Self::evaluate(self, expr_initializer))?;
            value = literal.unwrap();
        }
        self.environment.define(stmt.name.lexeme, value.clone());

        return Ok(value);
    }
    fn visit_while_stmt(&'a mut self, stmt: While) -> DefaultResult {
        let evaluated_condition = Self::evaluate(self, stmt.condition.clone())?;
        if Self::is_truthy(&evaluated_condition) {
            Self::execute(self, *stmt.body.clone())?;
        }
        return Ok(LiteralType::Nil);
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
    pub fn visit_logical_expr(&mut self, logical: Logical) -> DefaultResult {
        let left = Self::evaluate(self, *logical.left)?;

        if let TokenType::Or = logical.operator.ttype {
            if Self::is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !Self::is_truthy(&left) {
                return Ok(left);
            }
        }

        return Self::evaluate(self, *logical.right);
    }
    pub fn visit_grouping_expr(&mut self, grouping: Grouping) -> DefaultResult {
        Self::evaluate(self, *grouping.expression)
    }
    pub fn visit_unary_expr(&mut self, unary: Unary) -> DefaultResult {
        let right_r_value = Self::evaluate(self, *unary.right);

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
                return Ok(LiteralType::Bool(!Self::is_truthy(&right_value)));
            }
            _ => {}
        }
        Ok(LiteralType::Nil)
    }
    pub fn visit_variable_expr(&mut self, expr: Variable) -> DefaultResult {
        let get_result = self.environment.get(&expr.name);
        return get_result;
    }
    pub fn visit_assign_expr(&mut self, expr: Assign) -> DefaultResult {
        let value = Self::evaluate(self, *expr.value)?;
        let get_result = self.environment.assign(expr.name, value);
        return get_result;
    }
    pub fn visit_binary_expr(&mut self, binary: Binary) -> DefaultResult {
        let left = *binary.left;
        let right = *binary.right;
        let left_r_value = Self::evaluate(self, left);
        let right_r_value = Self::evaluate(self, right);

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
    pub fn is_truthy(item: &LiteralType) -> bool {
        match item {
            LiteralType::Bool(bool) => {
                return *bool;
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
