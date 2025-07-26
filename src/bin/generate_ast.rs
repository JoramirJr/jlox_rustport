use std::fs::File;
use std::io::Write;

pub struct Main();

impl Main {
    fn main(self) {
        Self::define_ast(
            "expr",
            Vec::from([
                "Assign: Token name, Box<ExpressionType> value",
                "Binary: Box<ExpressionType> left, Token operator, Box<ExpressionType> right",
                "Call: Box<ExpressionType> callee, Token paren, Vec<ExpressionType> arguments",
                "Grouping: Box<ExpressionType> expression",
                "Literal: LiteralType value",
                "Variable: Token name",
                "Logical: Box<ExpressionType> left, Token operator, Box<ExpressionType> right",
                "Unary: Token operator, Box<ExpressionType> right",
            ]),
        );
        Self::define_ast(
            "stmt",
            Vec::from([
                "Expression: ExpressionType expression",
                "Function: Token name, Vec<Token> params, Vec<StmtType> body",
                "If: Box<ExpressionType> condition, Box<StmtType> then_branch, Option<Box<StmtType>> else_branch",
                "Block: Vec<StmtType> statements",
                "Var: Token name, Option<ExpressionType> initializer",
                "Print: ExpressionType expression",
                "Return: Token keyword, ExpressionType value",
                "While: ExpressionType condition, Box<StmtType> body",
            ]),
        );
    }
    fn define_ast(basename: &str, types: Vec<&str>) {
        let path = format!("src/{}.rs", basename);

        let mut file_handler = File::create(&path).unwrap();

        if basename == "expr" {
            let _ = file_handler
                .write(format!("use crate::token_type::{{LiteralType, Token}};\n\n").as_bytes());
            let _ = file_handler.write(
                ["#[derive(Debug, Clone)]\n", "pub enum ExpressionType {\n"]
                    .concat()
                    .as_bytes(),
            );
        } else {
            let _ = file_handler.write(
                format!("use crate::{{expr::ExpressionType, token_type::Token}};\n\n").as_bytes(),
            );

            let _ = file_handler
                .write(format!("#[derive(Debug, Clone)]\npub enum StmtType {{\n").as_bytes());
        }
        Self::define_expr_stmt_type(&mut file_handler, &types);

        for t in &types {
            let (struct_name, fields) = t.split_once(":").unwrap();
            Self::define_type(&mut file_handler, struct_name.trim(), fields.trim());
        }
    }
    fn define_expr_stmt_type(file_handler: &mut File, field_list: &Vec<&str>) {
        for t in field_list {
            let (struct_name, _) = t.split_once(":").unwrap();

            let _ = file_handler.write(format!("    {0}({0}),\n", struct_name).as_bytes());
        }
        let _ = file_handler.write("}\n\n".as_bytes());
    }
    fn define_type(file_handler: &mut File, struct_name: &str, field_list: &str) {
        let _ = file_handler
            .write(format!("#[derive(Debug, Clone)]\npub struct {} {{\n", struct_name).as_bytes());

        let fields = field_list.split(", ");

        for field in fields {
            let field_split: Vec<&str> = field.split(" ").collect();
            let field_name = field_split[1];
            let field_type = field_split[0];
            let _ =
                file_handler.write(format!("    pub {}: {},\n", field_name, field_type).as_bytes());
        }

        let _ = file_handler.write(b"}\n");
    }
}

fn main() {
    let main = Main {};
    main.main();
}
