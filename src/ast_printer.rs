mod astPrinter {
    use std::ops::Deref;

    use jlox_rustport::{
        expr::{
            self,
            expr::{Binary, ExpressionGenericType, ExpressionType, Grouping, Literal, NonGenericExpressionType, Unary},
        },
        token_type::LiteralType,
    };
    pub struct AstPrinter();

    impl AstPrinter {
        fn print(expr: ExpressionType<ExpressionGenericType>) -> ExpressionType<ExpressionGenericType> {
            expr
        }
        fn visit_binary_expr(expr: Binary) -> String {
            return Self::parenthesize(expr.operator.lexeme, [&expr.left, &expr.right].to_vec());
        }
        fn visit_grouping_expr(expr: Grouping) -> String {
            return Self::parenthesize("group".to_string(), [&expr.expression].to_vec());
        }
        fn visit_literal_expr<T>(expr: Literal<LiteralType>) -> String {
            match expr.value {
                Some(value) => match value {
                    LiteralType::String(_) => "nil".to_string(),
                    LiteralType::F32(value) => value.to_string(),
                },
                None => "nil".to_string(),
            }
        }
        fn visit_unary_expr(expr: Unary) -> String {
            return Self::parenthesize(expr.operator.lexeme, [&expr.right].to_vec());
        }
        fn parenthesize(&self, name: String, exprs: Vec<&Box<NonGenericExpressionType>>) -> String {
            let mut builder = String::new();

            builder.push('(');
            builder.push_str(name.as_str());

            for expr in exprs {
                builder.push(' ');
                match expr.deref() {
                    ExpressionType::BinaryExpr(_expr) => builder.push_str(string),  
                }
            }
            builder.push(')');
            return builder;
        }
    }
}
