use crate::expr::ExpressionType;
use crate::token_type::LiteralType;
pub struct AstPrinter();

impl AstPrinter {
    pub fn print(expr: &ExpressionType) -> String {
        match expr {
            ExpressionType::Binary(expr) => {
                Self::parenthesize(&expr.operator.lexeme, [&expr.left, &expr.right].to_vec())
            }
            ExpressionType::Grouping(expr) => {
                Self::parenthesize(&"group".to_string(), [&expr.expression].to_vec())
            }
            ExpressionType::Literal(expr) => match &expr.value {
                LiteralType::Nil => "nil".to_string(),
                LiteralType::String(value) => value.clone(),
                LiteralType::Bool(value) => value.to_string().clone(),
                LiteralType::F32(value) => value.to_string(),
            },
            ExpressionType::Unary(expr) => {
                Self::parenthesize(&expr.operator.lexeme, [&expr.right].to_vec())
            }
            ExpressionType::Variable(_) => todo!(),
            ExpressionType::Assign(_) => todo!(),
            ExpressionType::Logical(_) => todo!(),
            ExpressionType::Call(_) => todo!(),
        }
    }
    pub fn parenthesize(name: &String, exprs: Vec<&Box<ExpressionType>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name.as_str());

        for expr in exprs {
            builder.push(' ');
            let sub_expr = Self::print(&**expr);
            builder.push_str(sub_expr.as_str());
        }
        builder.push(')');
        return builder;
    }
}
