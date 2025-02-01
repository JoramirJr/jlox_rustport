pub mod expr {
    use crate::token_type::{Token, TokenType};

    trait ExpressionBehaviors {
        fn interpret(&self) -> ();
        fn resolve(&self) -> ();
        fn analyze(&self) -> ();
    }

    pub enum ExpressionGenericType {
        Token(TokenType),
        Empty(())
    }

    pub enum ExpressionType<ExpressionGenericType> {
        BinaryExpr(Binary),
        UnaryExpr(Unary),
        GroupingExpr(Grouping),
        LiteralExpr(Literal<ExpressionGenericType>),
    }

    pub type NonGenericExpressionType = ExpressionType<ExpressionGenericType>;

    pub struct Binary {
        pub left: Box<NonGenericExpressionType>,
        pub operator: Token<String>,
        pub right: Box<NonGenericExpressionType>,
    }
    pub struct Grouping {
        pub expression: Box<NonGenericExpressionType>,
    }
    pub struct Literal<T> {
        pub value: Option<T>,
    }
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
}
