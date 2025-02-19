mod astPrinter {
    use jlox_rustport::expr::expr::{Binary, Grouping, Literal, Unary};
    pub struct AstPrinter();

    impl AstPrinter {
        fn sprint(self) -> AstPrinter {
            self
        }
        fn visit_binary_expr(expr: Binary) {
            return Self::parenthesize();
        }
        fn visit_grouping_expr(expr: Grouping) {
            return Self::parenthesize();
        }
        fn visit_literal_expr<T>(expr: Literal<T>) {
            return Self::parenthesize();
        }
        fn visit_unary_expr(expr: Unary) {
            return Self::parenthesize();
        }
        fn parenthesize() {
            return Self::parenthesize();
        }
    }
}
