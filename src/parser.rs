use std::sync::{LazyLock, Mutex};

use crate::expr::{Assign, Binary, ExpressionType, Grouping, Literal, Logical, Unary, Variable};
use crate::lox::Lox;
use crate::stmt::{Block, Expression, If, Print, StmtType, Var, While};
use crate::token_type::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
pub struct ParseError(String);

pub static PARSER_SINGLETON: LazyLock<Mutex<Parser>> = LazyLock::new(|| {
    Mutex::new(Parser {
        tokens: Vec::new(),
        current: 0,
    })
});

type DefaultResult = Result<StmtType, ParseError>;

impl Parser {
    pub fn parse(scanned_tokens: Vec<Token>) -> Vec<StmtType> {
        let parser_singleton = PARSER_SINGLETON.lock();

        match parser_singleton {
            Ok(mut parser) => {
                parser.tokens = scanned_tokens;
                let mut statements: Vec<StmtType> = Vec::new();

                while !Self::is_at_end(&parser) {
                    let declaration = Self::declaration(&mut parser);

                    if let Some(value) = declaration {
                        statements.push(value);
                    }
                }
                std::mem::drop(parser);
                statements
            }
            Err(err) => {
                panic!("Parser singleton lock unwrap failed; error: {:?}", err);
            }
        }
    }
    fn declaration(&mut self) -> Option<StmtType> {
        if Self::match_expr(self, &[TokenType::Var]) {
            let var_declaration = Self::var_declaration(self);

            match var_declaration {
                Ok(value) => return Some(value),
                Err(_) => {
                    Self::synchronize(self);
                    return None;
                }
            }
        }

        let stmt = Self::statement(self);

        match stmt {
            Ok(stmt) => {
                return Some(stmt);
            }
            Err(_) => {
                Self::synchronize(self);
                None
            }
        }
    }
    fn var_declaration(&mut self) -> DefaultResult {
        let name = Self::consume(self, &TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<ExpressionType> = None;

        if Self::match_expr(self, &[TokenType::Equal]) {
            let expr = Self::expression(self)?;
            initializer = Some(expr);
        }

        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;

        Ok(StmtType::VarExpr(Var {
            name: name,
            initializer: initializer,
        }))
    }
    fn while_statement(&mut self) -> DefaultResult {
        Self::consume(self, &TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = Self::expression(self)?;
        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after 'while' condition.",
        )?;

        let body = Self::statement(self)?;

        return Ok(StmtType::WhileExpr(While {
            condition,
            body: Box::new(body),
        }));
    }
    fn statement(&mut self) -> DefaultResult {
        return if Self::match_expr(self, &[TokenType::Print]) {
            Self::print_statement(self)
        } else if Self::match_expr(self, &[TokenType::LeftBrace]) {
            let statements = Self::block(self)?;
            Ok(StmtType::BlockExpr(Block { statements }))
        } else if Self::match_expr(self, &[TokenType::If]) {
            Self::if_statement(self)
        } else if Self::match_expr(self, &[TokenType::While]) {
            Self::while_statement(self)
        } else if Self::match_expr(self, &[TokenType::For]) {
            let stmt = Self::for_statement(self);
            return stmt;
        } else {
            Self::expression_statement(self)
        };
    }
    fn for_statement(&mut self) -> DefaultResult {
        Self::consume(self, &TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<StmtType> = None;

        if Self::match_expr(self, &[TokenType::Semicolon]) {
            initializer = None;
        } else if Self::match_expr(self, &[TokenType::Var]) {
            let var_decl = Self::var_declaration(self)?;
            initializer = Some(var_decl);
        } else {
            let expr_stmt = Self::expression_statement(self)?;
            initializer = Some(expr_stmt);
        }

        let mut condition: Option<ExpressionType> = None;

        if !Self::check(&self, &TokenType::Semicolon) {
            let expr_stmt = Self::expression(self)?;
            condition = Some(expr_stmt);
        }
        println!("Condition: {:?}", condition);
        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect closing ')' after loop condition.",
        )?;

        let mut increment: Option<ExpressionType> = None;

        if !Self::check(&self, &TokenType::RightParen) {
            let expr = Self::expression(self)?;
            increment = Some(expr);
        }

        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect closing ')' after 'for' clauses.",
        )?;

        let mut body = Self::statement(self)?;

        if let Some(increment) = increment {
            body = StmtType::BlockExpr(Block {
                statements: Vec::from([
                    body,
                    StmtType::ExpressionExpr(Expression {
                        expression: increment,
                    }),
                ]),
            });
        }

        if let None = condition {
            condition = Some(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Bool(true),
            }));
        }

        body = StmtType::WhileExpr(While {
            condition: condition.unwrap(),
            body: Box::new(body),
        });

        if let Some(initializer) = initializer {
            body = StmtType::BlockExpr(Block {
                statements: Vec::from([initializer, body]),
            })
        }

        return Ok(body);
    }
    fn if_statement(&mut self) -> DefaultResult {
        Self::consume(self, &TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = Self::expression(self)?;
        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after 'if' condition.",
        )?;

        let then_branch = Self::statement(self)?;

        if let StmtType::BlockExpr(then_block) = then_branch {
            let mut else_branch: Option<Block> = None;

            if Self::match_expr(self, &[TokenType::Else]) {
                let stmt_result = Self::statement(self)?;

                if let StmtType::BlockExpr(block) = stmt_result {
                    else_branch = Some(block);
                } else {
                    return Err(ParseError(
                        "Expect '{' after else condition's parens.".to_string(),
                    ));
                }
            }
            Ok(StmtType::IfExpr(If {
                condition: Box::new(condition),
                then_branch: then_block,
                else_branch,
            }))
        } else {
            Err(ParseError(
                "Expect '{' after if condition's parens.".to_string(),
            ))
        }
    }
    fn print_statement(&mut self) -> DefaultResult {
        let value: ExpressionType = Self::expression(self)?;

        Self::consume(self, &TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(StmtType::PrintExpr(Print { expression: value }))
    }
    fn expression_statement(&mut self) -> DefaultResult {
        let expr: ExpressionType = Self::expression(self)?;

        Self::consume(self, &TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(StmtType::PrintExpr(Print { expression: expr }))
    }
    fn block(&mut self) -> Result<Vec<StmtType>, ParseError> {
        let mut statements = Vec::new();

        while !Self::check(&self, &TokenType::RightBrace) && !Self::is_at_end(&self) {
            let declaration = Self::declaration(self);

            if let Some(decl) = declaration {
                statements.push(decl);
            }
        }
        Self::consume(self, &TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(statements);
    }
    fn assigment(&mut self) -> Result<ExpressionType, ParseError> {
        let expr = Self::or(self)?;

        if Self::match_expr(self, &[TokenType::Equal]) {
            let equals = Self::previous(&self);
            let value = Self::assigment(self)?;

            if let ExpressionType::VariableExpr(variable) = expr {
                let name = variable.name;
                return Ok(ExpressionType::AssignExpr(Assign {
                    name: name,
                    value: Box::new(value),
                }));
            }

            Err(ParseError(format!("{:?} Invalid assigment target", equals)))
        } else {
            return Ok(expr);
        }
    }
    pub fn or(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::and(self)?;

        while Self::match_expr(self, &[TokenType::Or]) {
            let operator = Self::previous(self);
            let right = Self::and(self)?;
            expr = ExpressionType::LogicalExpr(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }
    pub fn and(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::equality(self)?;

        while Self::match_expr(self, &[TokenType::And]) {
            let operator = Self::previous(self);
            let right = Self::equality(self)?;
            expr = ExpressionType::LogicalExpr(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }
    pub fn expression(&mut self) -> Result<ExpressionType, ParseError> {
        Self::assigment(self)
    }
    pub fn equality(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::comparison(self);

        while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        expr
    }
    pub fn comparison(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::term(self);

        while Self::match_expr(
            self,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        ) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn term(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::factor(self);
        while Self::match_expr(self, &[TokenType::Minus, TokenType::Plus]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn factor(&mut self) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::unary(self);

        while Self::match_expr(self, &[TokenType::Slash, TokenType::Star]) {
            let operator = Self::previous(&self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::BinaryExpr(Binary {
                                left: Box::new(ok_response),
                                operator: Token {
                                    ttype: operator.ttype,
                                    lexeme: operator.lexeme,
                                    literal: operator.literal,
                                    line: operator.line,
                                },
                                right: Box::new(right_expr),
                            }));
                        }
                        Err(err_response) => {
                            return Err(err_response);
                        }
                    }
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }

        return expr;
    }
    pub fn unary(&mut self) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::Bang, TokenType::Minus]) {
            let operator = Self::previous(&self);
            let right = Self::unary(self);

            match right {
                Ok(ok_response) => {
                    return Ok(ExpressionType::UnaryExpr(Unary {
                        operator: Token {
                            ttype: operator.ttype,
                            lexeme: operator.lexeme,
                            literal: operator.literal,
                            line: operator.line,
                        },
                        right: Box::new(ok_response),
                    }));
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        return Self::primary(self);
    }
    pub fn primary(&mut self) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::False]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Bool(false),
            }));
        }
        if Self::match_expr(self, &[TokenType::True]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Bool(true),
            }));
        }
        if Self::match_expr(self, &[TokenType::Nil]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: LiteralType::Nil,
            }));
        }

        if Self::match_expr(self, &[TokenType::Number, TokenType::String]) {
            return Ok(ExpressionType::LiteralExpr(Literal {
                value: Self::previous(&self).literal,
            }));
        }

        if Self::match_expr(self, &[TokenType::LeftParen]) {
            let expr = Self::expression(self);

            match expr {
                Ok(ok_response) => {
                    Self::consume(self, &TokenType::RightParen, "Expect ')' after expression")?;
                    return Ok(ExpressionType::GroupingExpr(Grouping {
                        expression: Box::new(ExpressionType::GroupingExpr(Grouping {
                            expression: Box::new(ok_response),
                        })),
                    }));
                }
                Err(err_response) => {
                    return Err(err_response);
                }
            }
        }
        if Self::match_expr(self, &[TokenType::Identifier]) {
            let prev_token = Self::previous(&self);
            return Ok(ExpressionType::VariableExpr(Variable { name: prev_token }));
        }
        Self::error(Self::peek(self), "Expect expression.");

        return Ok(ExpressionType::LiteralExpr(Literal {
            value: LiteralType::Nil,
        }));
    }
    pub fn consume(&mut self, t_type: &TokenType, message: &str) -> Result<Token, ParseError> {
        if !Self::check(self, t_type) {
            let next_token = Self::peek(self);
            return Err(Self::error(next_token, message));
        }
        Ok(Self::advance(self))
    }
    pub fn synchronize(&mut self) -> () {
        Self::advance(self);

        while !Self::is_at_end(self) {
            if Self::previous(self).ttype == TokenType::Semicolon {
                return;
            }
            match Self::peek(self).ttype {
                TokenType::Class => return,
                TokenType::For => return,
                TokenType::Fun => return,
                TokenType::If => return,
                TokenType::Print => return,
                TokenType::Return => return,
                TokenType::Var => return,
                TokenType::While => return,
                _ => {}
            }
            Self::advance(self);
        }
    }
    pub fn match_expr(&mut self, types: &[TokenType]) -> bool {
        types.iter().any(|t| {
            if Self::check(self, t) {
                Self::advance(self);
                return true;
            } else {
                return false;
            }
        })
    }
    pub fn check(&self, t_type: &TokenType) -> bool {
        if Self::is_at_end(self) {
            false
        } else {
            Self::peek(self).ttype == *t_type
        }
    }
    pub fn advance(&mut self) -> Token {
        if !Self::is_at_end(self) {
            self.current += 1;
        }
        Self::previous(self)
    }
    pub fn is_at_end(&self) -> bool {
        Self::peek(self).ttype == TokenType::Eof
    }
    pub fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    pub fn error(token: Token, message: &str) -> ParseError {
        Lox::error(token, message);
        ParseError("".to_string())
    }
}
