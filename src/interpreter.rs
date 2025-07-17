use std::{cell::RefCell, collections::HashMap, ops::Neg, rc::Rc};

use crate::{
    environment::{BindableValue, Environment}, expr::{Assign, Binary, Call, ExpressionType, Grouping, Literal, Logical, Unary, Variable}, lox::Lox, lox_function::LoxFunction, stmt::{Block, Function, If, StmtType, Var, While}, token_type::{LiteralType, Token, TokenType}, LoxCallable, lox_std::{NativeFunction}
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Option<Rc<RefCell<Environment>>>,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

type DefaultResult = Result<Option<BindableValue>, RuntimeError>;

impl Interpreter {
    pub fn interpret(mut self, statements: Vec<StmtType>, lox_strt_instance: &mut Lox) -> () {
        for statement in statements {
            let execute_result = Self::execute(&mut self, statement);

            if let Err(runtime_error) = execute_result {
                lox_strt_instance.runtime_error(runtime_error);
                break;
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
            ExpressionType::Call(_) => todo!(),
        }
    }
    fn execute(&mut self, stmt: StmtType) -> DefaultResult {
        match stmt {
            StmtType::Expression(expr) => Self::visit_expression_stmt(self, expr.expression),
            StmtType::Print(print) => Self::visit_print_stmt(self, print.expression),
            StmtType::Var(var) => Self::visit_var_stmt(self, var),
            StmtType::Block(block) => Self::visit_block_stmt(self, block),
            StmtType::If(if_stmt) => Self::visit_if_stmt(self, if_stmt),
            StmtType::While(while_stmt) => Self::visit_while_stmt(self, while_stmt),
            StmtType::Function(_function) => todo!(),
        }
    }
    fn visit_block_stmt(&mut self, stmt: Block) -> DefaultResult {
        Self::execute_block(self, stmt.statements)
    }
    pub fn execute_block(&mut self, statements: Vec<StmtType>) -> DefaultResult {
        let environment = Environment {
            enclosing: Some(self.environment.clone().unwrap()),
            values: HashMap::new(),
        };
        let previous = self.environment.clone();

        self.environment = Some(Rc::new(RefCell::new(environment)));

        let mut curr_execute_result: Option<BindableValue> =
            Some(BindableValue::Literal(LiteralType::Nil));

        for statement in statements {
            let execute_result: DefaultResult = Self::execute(self, statement);

            match execute_result {
                Ok(value) => {
                    curr_execute_result = value;
                }
                Err(err) => {
                    self.environment = previous;
                    return Err(err);
                }
            };
        }
        self.environment = previous;
        Ok(curr_execute_result)
    }
    fn visit_expression_stmt(&mut self, expr: ExpressionType) -> DefaultResult {
        Self::evaluate(self, expr)
    }
    fn visit_function_stmt(&mut self, stmt: Function) -> DefaultResult {
        let lexeme = stmt.name.lexeme.clone();

        let function = LoxFunction { declaration: stmt };
        self.environment.clone().unwrap().borrow_mut().define(lexeme, BindableValue::Function(function));

        Ok(None)
    }
    fn visit_if_stmt(&mut self, stmt: If) -> DefaultResult {
        let evaluate_result: Option<BindableValue> = Self::evaluate(self, *stmt.condition)?;

        if let Some(value) = evaluate_result {
            if Self::is_truthy(&value) {
                Self::execute(self, StmtType::Block(stmt.then_branch))
            } else if let Some(else_branch) = stmt.else_branch {
                Self::execute(self, StmtType::Block(else_branch))
            } else {
                Ok(Some(BindableValue::Literal(LiteralType::Nil)))
            }
        } else {
            panic!(
                "Interpreter implementation fail - if stmt condition not evaluated to a valid value"
            )
        }
    }
    fn visit_print_stmt(&mut self, expr: ExpressionType) -> DefaultResult {
        let value = Self::evaluate(self, expr);

        match value {
            Ok(value) => {
                println!("{:?}", Self::stringify(&Option::expect(value, "Interpreter implementation fail - print stmt adjacent expression not evaluated to a valid value")));
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
    fn visit_var_stmt(&mut self, stmt: Var) -> DefaultResult {
        let mut value: BindableValue = BindableValue::Literal(LiteralType::Nil);

        if let Some(expr_initializer) = stmt.initializer {
            let bindable = Self::evaluate(self, expr_initializer)?;
            value = bindable.unwrap();
        }
        self.environment
            .unwrap()
            .borrow_mut()
            .define(stmt.name.lexeme, value);

        return Ok(None);
    }
    fn visit_while_stmt(&mut self, stmt: While) -> DefaultResult {
        while Self::is_truthy(&Option::expect(
            Self::evaluate(self, stmt.condition.clone())?,
            "Interpreter implementation fail - while stmt condition not evaluated to a valid value",
        )) {
            Self::execute(self, *stmt.body.clone())?;
        }
        return Ok(None);
    }
    pub fn stringify(value: &BindableValue) -> String {
        match value {
            BindableValue::Literal(LiteralType::F32(f32_value)) => {
                        let mut text = f32_value.to_string();
                        if text.ends_with(".0") {
                            let decimal_offset = text.find(".0").unwrap_or(text.len());
                            text = text.drain(..decimal_offset).collect();
                        }
                        text
                    }
            BindableValue::Literal(LiteralType::Nil) => "nil".to_string(),
            BindableValue::Literal(LiteralType::Bool(bool_value)) => bool_value.to_string(),
            BindableValue::Literal(LiteralType::String(string_value)) => string_value.clone(),
            BindableValue::Function(lox_function) => lox_function.to_string(),
            BindableValue::NativeFunction(native_function) => {
                match native_function {
                    NativeFunction::Clock(clock) => {
                        clock.to_string()
                    }
                }
            },
        }
    }
    pub fn visit_literal_expr(literal: Literal) -> DefaultResult {
        Ok(Some(BindableValue::Literal(literal.value)))
    }
    pub fn visit_logical_expr(&mut self, logical: Logical) -> DefaultResult {
        let left = Self::evaluate(self, *logical.left)?;

        if let TokenType::Or = logical.operator.ttype {
            if Self::is_truthy(&Option::expect(
            left.clone(),
            "Interpreter implementation fail - while stmt condition not evaluated to a valid value",
        )) {
                return Ok(left);
            }
        } else {
            if !Self::is_truthy(&Option::expect(
            left.clone(),
            "Interpreter implementation fail - while stmt condition not evaluated to a valid value",
        )) {
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
                if let Some(BindableValue::Literal(LiteralType::F32(f32_value))) = right_value {
                    return Ok(Some(BindableValue::Literal(LiteralType::F32(
                        f32_value.neg(),
                    ))));
                } else {
                    return Err(RuntimeError {
                        message: String::from("Operand must be a number"),
                        token: unary.operator,
                    });
                }
            }
            TokenType::Bang => {
                return Ok(Some(BindableValue::Literal(LiteralType::Bool(
                    !Self::is_truthy(&Option::expect(
            right_value,
            "Interpreter implementation fail - while stmt condition not evaluated to a valid value",
        )),
                ))));
            }
            _ => {}
        }
        Ok(None)
    }
    pub fn visit_variable_expr(&mut self, expr: Variable) -> DefaultResult {
        let get_result = self.environment.borrow_mut().get(&expr.name)?;
        Ok(Some(get_result))
    }
    pub fn visit_assign_expr(&mut self, expr: Assign) -> DefaultResult {
        let value = Self::evaluate(self, *expr.value)?;
        let get_result = self.environment.borrow_mut().assign(
            expr.name,
            Option::expect(
                value,
                "Interpreter implementation fail - assignment value not evaluated to a valid value",
            ),
        )?;
        Ok(Some(get_result))
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

        let left_unwrapped_value = Option::expect(left_value, 
                      "Interpreter implementation fail - left operator in binary expression not evaluated to a valid value");

        let right_unwrapped_value = Option::expect(right_value, 
                      "Interpreter implementation fail - left operator in binary expression not evaluated to a valid value");

        if let TokenType::Minus = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(
                        BindableValue::Literal(LiteralType::F32(f32_left - f32_right))
                    ));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::Plus = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::F32(f32_left + f32_right))));
                }
                (BindableValue::Literal(LiteralType::String(string_left)) , BindableValue::Literal(LiteralType::String(string_right)) ) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::String(format!(
                        "{}{}",
                        string_left, string_right
                    )))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be two numbers or two strings"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::Slash = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::F32(f32_left / f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::Star = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::F32(f32_left * f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::Greater = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::Bool(f32_left > f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::GreaterEqual = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::Bool(f32_left >= f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::Less = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::Bool(f32_left < f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::LessEqual = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(LiteralType::F32(f32_left)), BindableValue::Literal(LiteralType::F32(f32_right))) => {
                    return Ok(Some(BindableValue::Literal(LiteralType::Bool(f32_left <= f32_right))));
                }
                _ => {
                    return Err(RuntimeError {
                        message: String::from("Operands must be a number"),
                        token: binary.operator,
                    })
                }
            }
        } else if let TokenType::BangEqual = binary.operator.ttype {
            match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(left_literal), BindableValue::Literal(right_literal)) => {
                  Ok(Some(BindableValue
                            ::Literal(LiteralType::Bool(left_literal != right_literal))))  
                }
                _ => {
            return Err(RuntimeError {
                message: String::from("Comparisons only allowed between literals"),
                token: binary.operator,
            });        
                }
            }
            
        } else if let TokenType::EqualEqual = binary.operator.ttype {
             match (left_unwrapped_value, right_unwrapped_value) {
                (BindableValue::Literal(left_literal), BindableValue::Literal(right_literal)) => {
                  Ok(Some(BindableValue
                            ::Literal(LiteralType::Bool(left_literal == right_literal))))  
                }
                _ => {
            return Err(RuntimeError {
                message: String::from("Comparisons only allowed between literals"),
                token: binary.operator,
            });        
                }
            }
        } else {
            return Err(RuntimeError {
                message: String::from("Invalid operator"),
                token: binary.operator,
            });
        }
    }
     pub fn visit_call_expr(&mut self, expr: Call) -> DefaultResult {
         let callee = Self::evaluate(self, *expr.callee)?.unwrap();

         let mut arguments: Vec<BindableValue> = Vec::new();

         for argument in expr.arguments {
             arguments.push(Option::expect(Self::evaluate(self, argument)?, "Bug in visit_call_expr() call"));
         }

         match callee {
            BindableValue::Function(function) => {
                                function.call(Some(self), arguments);
                                Ok(None)
                             }
            BindableValue::Literal(_) => {
                        Err(RuntimeError { token: expr.paren, message: "Can only call functions and classes.".to_string()
                     })
                    },
BindableValue::NativeFunction(_native_function) => todo!(),
                    }
     }
    pub fn is_truthy(item: &BindableValue) -> bool {
        match item {
            BindableValue::Literal(LiteralType::Bool(bool)) => {
                return *bool;
            }
            BindableValue::Literal(LiteralType::Nil) => {
                return false;
            }
            _ => {
                return true;
            }
        }
    }
}
