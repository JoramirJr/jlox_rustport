pub mod expr {

    use crate::token_type::{LiteralType, Token, TokenType};

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
    pub struct Literal<LiteralType> {
        pub value: Option<LiteralType>,
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
    impl Binary {
        fn accept(self) -> Binary {
            self
        }
    }
    impl Unary {
        fn accept(self) -> Unary {
            self
        }
    }
    impl Grouping {
        fn accept(self) -> Grouping {
            self
        }
    }
    impl<T> Literal<T> {
        fn accept(self) -> Literal<T> {
            self
        }
    }
}
