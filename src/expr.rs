pub mod expr {
    use crate::token_type::{Token, TokenType};

    trait ExpressionBehaviors {
        fn interpret(&self) -> ();
        fn resolve(&self) -> ();
        fn analyze(&self) -> ();
    }

    #[derive(Debug)]
    pub enum ExpressionGenericType {
        Token(TokenType),
        Empty(()),
    }

    pub trait VisitorMethods {
        fn accept(visitor: Visitor<T>) -> ();
    }

    pub enum Visitor<T> {
        VisitBinary(Binary),
        VisitGrouping(Grouping),
        VisitLiteral(Literal<T>),
        VisitUnary(Unary),
    }

    #[derive(Debug)]
    pub enum ExpressionType<ExpressionGenericType> {
        BinaryExpr(Binary),
        GroupingExpr(Grouping),
        LiteralExpr(Literal<ExpressionGenericType>),
        UnaryExpr(Unary),
    }
    pub type NonGenericExpressionType = ExpressionType<ExpressionGenericType>;
    #[derive(Debug)]
    pub struct Binary {
        pub left: Box<NonGenericExpressionType>,
        pub operator: Token<String>,
        pub right: Box<NonGenericExpressionType>,
    }
    #[derive(Debug)]
    pub struct Grouping {
        pub expression: Box<NonGenericExpressionType>,
    }
    #[derive(Debug)]
    pub struct Literal<T> {
        pub value: Option<T>,
    }
    #[derive(Debug)]
    pub struct Unary {
        pub operator: Token<String>,
        pub right: Box<NonGenericExpressionType>,
    }
    impl ExpressionBehaviors for Binary {
        fn interpret(&self) -> () {
            ()
        }
        fn resolve(&self) -> () {
            ()
        }
        fn analyze(&self) -> () {
            ()
        }
    }
    impl ExpressionBehaviors for Grouping {
        fn interpret(&self) -> () {
            ()
        }
        fn resolve(&self) -> () {
            ()
        }
        fn analyze(&self) -> () {
            ()
        }
    }
    impl<T> ExpressionBehaviors for Literal<T> {
        fn interpret(&self) -> () {
            ()
        }
        fn resolve(&self) -> () {
            ()
        }
        fn analyze(&self) -> () {
            ()
        }
    }
    impl ExpressionBehaviors for Unary {
        fn interpret(&self) -> () {
            ()
        }
        fn resolve(&self) -> () {
            ()
        }
        fn analyze(&self) -> () {
            ()
        }
    }
    impl VisitorMethods for Binary {
        fn accept(visitor: Visitor<T>) -> () {
            Visitor::VisitBinary(self);
        }
    }
    impl VisitorMethods for Grouping {
        fn accept(visitor: Visitor<T>) -> () {
            Visitor::VisitGrouping(self);
        }
    }
    impl VisitorMethods for Literal<T> {
        fn accept(visitor: Visitor<T>) -> () {
            Visitor::VisitLiteral<T>(self);
        }
    }
    impl VisitorMethods for Unary {
        fn accept(visitor: Visitor<T>) -> () { 
            Visitor::VisitUnary(self);
        }
    }
}
