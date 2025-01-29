pub mod expr {
    use crate::token_type::Token;

    trait ExpressionBehaviors {
        fn interpret(&self) -> ();
        fn resolve(&self) -> ();
        fn analyze(&self) -> ();
    }

    pub enum ExpressionType<T> {
        BinaryExpr(Binary<T>),
        UnaryExpr(Unary<T>),
        GroupingExpr(Grouping),
        LiteralExpr(Literal<T>),
    }

    pub struct Binary<T> {
        pub left: Box<ExpressionType<T>>,
        pub operator: Token<String>,
        pub right: Box<ExpressionType<T>>,
    }
    pub struct Grouping {
        pub expression: String,
    }
    pub struct Literal<T> {
        pub value: Option<T>,
    }
    pub struct Unary<T> {
        pub operator: Token<String>,
        pub right: Box<ExpressionType<T>>,
    }
    impl<T> ExpressionBehaviors for Binary<T> {
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
    impl<T> ExpressionBehaviors for Unary<T> {
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
