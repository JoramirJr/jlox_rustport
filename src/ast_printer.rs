use crate::expr::{self, Binary, ExpressionType, Grouping, Literal, Unary};
use crate::token_type::LiteralType;
pub struct AstPrinter();

impl AstPrinter {
    pub fn print(expr: ExpressionType) -> String {
        match expr {
            ExpressionType::BinaryExpr(expr) => {
                Self::parenthesize(expr.operator.lexeme, [&expr.left, &expr.right].to_vec())
            }
            ExpressionType::GroupingExpr(expr) => {
                Self::parenthesize("group".to_string(), [&expr.expression].to_vec())
            }
            ExpressionType::LiteralExpr(expr) => match expr.value {
                LiteralType::Nil => "nil".to_string(),
                _ => expr.value.to_string(),
            },
            ExpressionType::UnaryExpr(expr) => {
                Self::parenthesize(expr.operator.lexeme, [&expr.right].to_vec())
            }
        }
    }
    // fn visit_binary_expr(expr: Binary) -> String {
    //     return Self::parenthesize(
    //         self,
    //         expr.operator.lexeme,
    //         [&expr.left, &expr.right].to_vec(),
    //     );
    // }
    // fn visit_grouping_expr(expr: Grouping) -> String {
    //     return Self::parenthesize(self, "group".to_string(), [&expr.expression].to_vec());
    // }
    // fn visit_literal_expr(expr: Literal) -> String {
    //     match expr.value {
    //         LiteralType::Nil => "nil".to_string(),
    //         _ => expr.value.to_string(),
    //     }
    // }
    // fn visit_unary_expr(expr: Unary) -> String {
    //     return Self::parenthesize(self, expr.operator.lexeme, [&expr.right].to_vec());
    // }
    fn parenthesize(name: String, exprs: Vec<&Box<ExpressionType>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name.as_str());

        for expr in exprs {
            builder.push(' ');
            let expression_type = expr.as_ref();
            if let ExpressionType::LiteralExpr(lit_expr) = expression_type {
                match lit_expr.value.clone() {
                    LiteralType::Bool(value) => {
                        builder.push_str(value.to_string().as_str());
                    }
                    LiteralType::String(value) => {
                        builder.push_str(value.as_str());
                    }
                    LiteralType::F32(value) => {
                        builder.push_str(value.to_string().as_str());
                    }
                    LiteralType::Nil => {}
                }
            }
        }
        builder.push(')');
        return builder;
    }
}
