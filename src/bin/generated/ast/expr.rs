use crate::token_type::Token;

mod expr {

    trait ExpressionBehaviors {
        fn interpret(&self) -> ();
        fn resolve(&self) -> ();
        fn analyze(&self) -> ();
    }

    struct Binary {
        left: String,
        operator: Token,
        right: String,
    }
    struct Grouping {
        expression: String,
    }
    struct Literal<T> {
        value: Option<T>,
    }
    struct Unary {
        operator: Token,
        right: String,
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
    impl ExpressionBehaviors for Literal<T> {
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
