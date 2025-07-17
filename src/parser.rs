use crate::expr::{
    Assign, Binary, Call, ExpressionType, Grouping, Literal, Logical, Unary, Variable,
};
use crate::lox::Lox;
use crate::stmt::{Block, Expression, Function, If, Print, StmtType, Var, While};
use crate::token_type::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
pub struct ParseError(String);

type DefaultResult = Result<StmtType, ParseError>;

impl Parser {
    pub fn parse(
        &mut self,
        scanned_tokens: Vec<Token>,
        lox_strt_instance: &mut Lox,
    ) -> Vec<StmtType> {
        self.tokens = scanned_tokens;
        let mut statements: Vec<StmtType> = Vec::new();

        while !Self::is_at_end(&self) {
            let declaration = Self::declaration(self, lox_strt_instance);

            if let Ok(value) = declaration {
                statements.push(value);
            }
        }
        statements
    }
    fn declaration(&mut self, lox_strt_instance: &mut Lox) -> Result<StmtType, ParseError> {
        if Self::match_expr(self, &[TokenType::Var]) {
            let var_declaration = Self::var_declaration(self, lox_strt_instance);

            match var_declaration {
                Ok(value) => return Ok(value),
                Err(_) => {
                    Self::synchronize(self);
                    return Err(ParseError("".to_string()));
                }
            }
        } else if Self::match_expr(self, &[TokenType::Fun]) {
            return Ok(Self::function(self, "function", lox_strt_instance))?;
        }

        let stmt = Self::statement(self, lox_strt_instance);

        match stmt {
            Ok(stmt) => {
                return Ok(stmt);
            }
            Err(_) => {
                Self::synchronize(self);
                return Err(ParseError("".to_string()));
            }
        }
    }
    fn var_declaration(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        let name = Self::consume(
            self,
            &TokenType::Identifier,
            "Expect variable name.",
            lox_strt_instance,
        )?;

        let mut initializer: Option<ExpressionType> = None;

        if Self::match_expr(self, &[TokenType::Equal]) {
            let expr = Self::expression(self, lox_strt_instance)?;
            initializer = Some(expr);
        }

        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect ';' after variable declaration",
            lox_strt_instance,
        )?;

        Ok(StmtType::Var(Var {
            name: name,
            initializer: initializer,
        }))
    }
    fn while_statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        Self::consume(
            self,
            &TokenType::LeftParen,
            "Expect '(' after 'while'.",
            lox_strt_instance,
        )?;
        let condition = Self::expression(self, lox_strt_instance)?;
        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after 'while' condition.",
            lox_strt_instance,
        )?;

        let body = Self::statement(self, lox_strt_instance)?;

        return Ok(StmtType::While(While {
            condition,
            body: Box::new(body),
        }));
    }
    fn statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        return if Self::match_expr(self, &[TokenType::Print]) {
            Self::print_statement(self, lox_strt_instance)
        } else if Self::match_expr(self, &[TokenType::LeftBrace]) {
            let statements = Self::block(self, lox_strt_instance)?;
            Ok(StmtType::Block(Block { statements }))
        } else if Self::match_expr(self, &[TokenType::If]) {
            Self::if_statement(self, lox_strt_instance)
        } else if Self::match_expr(self, &[TokenType::While]) {
            Self::while_statement(self, lox_strt_instance)
        } else if Self::match_expr(self, &[TokenType::For]) {
            let stmt = Self::for_statement(self, lox_strt_instance);
            return stmt;
        } else {
            Self::expression_statement(self, lox_strt_instance)
        };
    }
    fn for_statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        Self::consume(
            self,
            &TokenType::LeftParen,
            "Expect '(' after 'for'.",
            lox_strt_instance,
        )?;

        let mut initializer: Option<StmtType> = None;

        if Self::match_expr(self, &[TokenType::Semicolon]) {
            initializer = None;
        } else if Self::match_expr(self, &[TokenType::Var]) {
            let var_decl = Self::var_declaration(self, lox_strt_instance)?;
            initializer = Some(var_decl);
        } else {
            let expr_stmt = Self::expression_statement(self, lox_strt_instance)?;
            initializer = Some(expr_stmt);
        }

        let mut condition: Option<ExpressionType> = None;

        if !Self::check(&self, &TokenType::Semicolon) {
            let expr_stmt = Self::expression(self, lox_strt_instance)?;
            condition = Some(expr_stmt);
        }
        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect closing ')' after loop condition.",
            lox_strt_instance,
        )?;

        let mut increment: Option<ExpressionType> = None;

        if !Self::check(&self, &TokenType::RightParen) {
            let expr = Self::expression(self, lox_strt_instance)?;
            increment = Some(expr);
        }

        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect closing ')' after 'for' clauses.",
            lox_strt_instance,
        )?;

        let mut body = Self::statement(self, lox_strt_instance)?;

        if let Some(increment) = increment {
            body = StmtType::Block(Block {
                statements: Vec::from([
                    body,
                    StmtType::Expression(Expression {
                        expression: increment,
                    }),
                ]),
            });
        }

        if let None = condition {
            condition = Some(ExpressionType::Literal(Literal {
                value: LiteralType::Bool(true),
            }));
        }

        body = StmtType::While(While {
            condition: condition.unwrap(),
            body: Box::new(body),
        });

        if let Some(initializer) = initializer {
            body = StmtType::Block(Block {
                statements: Vec::from([initializer, body]),
            })
        }

        return Ok(body);
    }
    fn if_statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        Self::consume(
            self,
            &TokenType::LeftParen,
            "Expect '(' after 'if'.",
            lox_strt_instance,
        )?;
        let condition = Self::expression(self, lox_strt_instance)?;
        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after 'if' condition.",
            lox_strt_instance,
        )?;

        let then_branch = Self::statement(self, lox_strt_instance)?;

        if let StmtType::Block(then_block) = then_branch {
            let mut else_branch: Option<Block> = None;

            if Self::match_expr(self, &[TokenType::Else]) {
                let stmt_result = Self::statement(self, lox_strt_instance)?;

                if let StmtType::Block(block) = stmt_result {
                    else_branch = Some(block);
                } else {
                    return Err(ParseError(
                        "Expect '{' after else condition's parens.".to_string(),
                    ));
                }
            }
            Ok(StmtType::If(If {
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
    fn print_statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        let value: ExpressionType = Self::expression(self, lox_strt_instance)?;

        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect ';' after value.",
            lox_strt_instance,
        )?;

        Ok(StmtType::Print(Print { expression: value }))
    }
    fn expression_statement(&mut self, lox_strt_instance: &mut Lox) -> DefaultResult {
        let expr: ExpressionType = Self::expression(self, lox_strt_instance)?;

        Self::consume(
            self,
            &TokenType::Semicolon,
            "Expect ';' after expression.",
            lox_strt_instance,
        )?;

        Ok(StmtType::Expression(Expression { expression: expr }))
    }
    fn function(
        &mut self,
        kind: &str,
        lox_strt_instance: &mut Lox,
    ) -> Result<StmtType, ParseError> {
        let name = Self::consume(
            self,
            &TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
            lox_strt_instance,
        )?;

        Self::consume(
            self,
            &TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
            lox_strt_instance,
        )?;

        let mut params: Vec<Token> = Vec::new();

        if !Self::check(&self, &TokenType::RightParen) {
            if params.len() > 255 {
                return Err(Self::error(
                    Self::peek(&self),
                    "Can't have more than 255 characters",
                    lox_strt_instance,
                ));
            }
            params.push(Self::consume(
                    self,
                    &TokenType::Identifier,
                    "Expect parameter name.",
                    lox_strt_instance,
                )?);
            while Self::match_expr(self, &[TokenType::Comma]) {
                params.push(Self::consume(
                    self,
                    &TokenType::Identifier,
                    "Expect parameter name.",
                    lox_strt_instance,
                )?)
            }
        }

        Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after parameters.",
            lox_strt_instance,
        )?;

        Self::consume(
            self,
            &TokenType::LeftBrace,
            format!("Expect ')' before {} body", kind).as_str(),
            lox_strt_instance,
        )?;

        let body: Vec<StmtType> = Self::block(self, lox_strt_instance)?;

        return Ok(StmtType::Function(Function { name, params, body }));
    }
    fn block(&mut self, lox_strt_instance: &mut Lox) -> Result<Vec<StmtType>, ParseError> {
        let mut statements = Vec::new();

        while !Self::check(&self, &TokenType::RightBrace) && !Self::is_at_end(&self) {
            let declaration = Self::declaration(self, lox_strt_instance);

            if let Ok(decl) = declaration {
                statements.push(decl);
            }
        }
        Self::consume(
            self,
            &TokenType::RightBrace,
            "Expect '}' after block.",
            lox_strt_instance,
        )?;
        return Ok(statements);
    }
    fn assigment(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let expr = Self::or(self, lox_strt_instance)?;

        if Self::match_expr(self, &[TokenType::Equal]) {
            let equals = Self::previous(&self);
            let value = Self::assigment(self, lox_strt_instance)?;

            if let ExpressionType::Variable(variable) = expr {
                let name = variable.name;
                return Ok(ExpressionType::Assign(Assign {
                    name: name,
                    value: Box::new(value),
                }));
            }

            Err(ParseError(format!("{:?} Invalid assigment target", equals)))
        } else {
            return Ok(expr);
        }
    }
    pub fn or(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::and(self, lox_strt_instance)?;

        while Self::match_expr(self, &[TokenType::Or]) {
            let operator = Self::previous(self);
            let right = Self::and(self, lox_strt_instance)?;
            expr = ExpressionType::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }
    pub fn and(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::equality(self, lox_strt_instance)?;

        while Self::match_expr(self, &[TokenType::And]) {
            let operator = Self::previous(self);
            let right = Self::equality(self, lox_strt_instance)?;
            expr = ExpressionType::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }
    pub fn expression(
        &mut self,
        lox_strt_instance: &mut Lox,
    ) -> Result<ExpressionType, ParseError> {
        Self::assigment(self, lox_strt_instance)
    }
    pub fn equality(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::comparison(self, lox_strt_instance);

        while Self::match_expr(self, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::comparison(self, lox_strt_instance);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::Binary(Binary {
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
    pub fn comparison(
        &mut self,
        lox_strt_instance: &mut Lox,
    ) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::term(self, lox_strt_instance);

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
                    let right = Self::unary(self, lox_strt_instance);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::Binary(Binary {
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
    pub fn term(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::factor(self, lox_strt_instance);
        while Self::match_expr(self, &[TokenType::Minus, TokenType::Plus]) {
            let operator = Self::previous(self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self, lox_strt_instance);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::Binary(Binary {
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
    pub fn factor(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::unary(self, lox_strt_instance);

        while Self::match_expr(self, &[TokenType::Slash, TokenType::Star]) {
            let operator = Self::previous(&self);

            match expr {
                Ok(ok_response) => {
                    let right = Self::unary(self, lox_strt_instance);
                    match right {
                        Ok(right_expr) => {
                            expr = Ok(ExpressionType::Binary(Binary {
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
    pub fn unary(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::Bang, TokenType::Minus]) {
            let operator = Self::previous(&self);
            let right = Self::unary(self, lox_strt_instance);

            match right {
                Ok(ok_response) => {
                    return Ok(ExpressionType::Unary(Unary {
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
        return Self::call(self, lox_strt_instance);
    }
    fn finish_call(
        &mut self,
        callee: ExpressionType,
        lox_strt_instance: &mut Lox,
    ) -> Result<ExpressionType, ParseError> {
        let mut arguments: Vec<ExpressionType> = Vec::new();

        if !Self::check(&self, &TokenType::RightParen) {
            if arguments.len() >= 255 {
                lox_strt_instance.error(Self::peek(&self), "Can't have more than 255 arguments.");
            }

            arguments.push(Self::expression(self, lox_strt_instance)?);
            while Self::match_expr(self, &[TokenType::Comma]) {
                arguments.push(Self::expression(self, lox_strt_instance)?);
            }
        }

        let paren = Self::consume(
            self,
            &TokenType::RightParen,
            "Expect ')' after arguments.",
            lox_strt_instance,
        )?;

        return Ok(ExpressionType::Call(Call {
            callee: Box::new(callee),
            arguments,
            paren,
        }));
    }
    pub fn call(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        let mut expr = Self::primary(self, lox_strt_instance)?;

        loop {
            if Self::match_expr(self, &[TokenType::LeftParen]) {
                expr = Self::finish_call(self, expr, lox_strt_instance)?;
            } else {
                break;
            }
        }

        return Ok(expr);
    }
    pub fn primary(&mut self, lox_strt_instance: &mut Lox) -> Result<ExpressionType, ParseError> {
        if Self::match_expr(self, &[TokenType::False]) {
            return Ok(ExpressionType::Literal(Literal {
                value: LiteralType::Bool(false),
            }));
        }
        if Self::match_expr(self, &[TokenType::True]) {
            return Ok(ExpressionType::Literal(Literal {
                value: LiteralType::Bool(true),
            }));
        }
        if Self::match_expr(self, &[TokenType::Nil]) {
            return Ok(ExpressionType::Literal(Literal {
                value: LiteralType::Nil,
            }));
        }

        if Self::match_expr(self, &[TokenType::Number, TokenType::String]) {
            return Ok(ExpressionType::Literal(Literal {
                value: Self::previous(&self).literal.unwrap(),
            }));
        }

        if Self::match_expr(self, &[TokenType::LeftParen]) {
            let expr = Self::expression(self, lox_strt_instance);

            match expr {
                Ok(ok_response) => {
                    Self::consume(
                        self,
                        &TokenType::RightParen,
                        "Expect ')' after expression",
                        lox_strt_instance,
                    )?;
                    return Ok(ExpressionType::Grouping(Grouping {
                        expression: Box::new(ExpressionType::Grouping(Grouping {
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
            return Ok(ExpressionType::Variable(Variable { name: prev_token }));
        }
        Self::error(Self::peek(self), "Expect expression.", lox_strt_instance);

        return Ok(ExpressionType::Literal(Literal {
            value: LiteralType::Nil,
        }));
    }
    pub fn consume(
        &mut self,
        t_type: &TokenType,
        message: &str,
        lox_strt_instance: &mut Lox,
    ) -> Result<Token, ParseError> {
        if !Self::check(self, t_type) {
            let next_token = Self::peek(self);
            return Err(Self::error(next_token, message, lox_strt_instance));
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
    pub fn error(token: Token, message: &str, lox_strt_instance: &mut Lox) -> ParseError {
        lox_strt_instance.error(token, message);
        ParseError("".to_string())
    }
}
