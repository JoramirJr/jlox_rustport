use std::fs::File;
use std::io::Write;

pub struct Main();

impl Main {
    fn main(self) {
        Self::define_ast(
            "expr",
            Vec::from([
                "Binary: Box<ExpressionType> left, Token operator, Box<ExpressionType> right",
                "Grouping: Box<ExpressionType> expression",
                "Literal: LiteralType value",
                "Unary: Token operator, Box<ExpressionType> right",
            ]),
        );
        Self::define_ast(
            "stmt",
            Vec::from([
                "Expression: ExpressionType expression",
                "Print: ExpressionType expression",
            ]),
        );
    }
    fn define_ast(basename: &str, types: Vec<&str>) {
        let path = ["src", "/", basename, ".rs"].concat();

        let mut file_handler = File::create(&path).unwrap();

        if basename == "expr" {
            let _ = file_handler.write(
                ["use crate::token_type::{LiteralType, Token};\n\n"]
                    .concat()
                    .as_bytes(),
            );
            let _ = file_handler.write(
                ["#[derive(Debug)]\n", "pub enum ExpressionType {\n"]
                    .concat()
                    .as_bytes(),
            );
        } else {
            let _ =
                file_handler.write(["use crate::expr::ExpressionType;\n\n"].concat().as_bytes());

            let _ = file_handler.write(
                ["#[derive(Debug)]\n", "pub enum StmtType {\n"]
                    .concat()
                    .as_bytes(),
            );
        }
        Self::define_expr_type(&mut file_handler, &types);

        for t in &types {
            let (struct_name, fields) = t.split_once(":").unwrap();
            Self::define_type(&mut file_handler, struct_name.trim(), fields.trim());
        }
    }
    fn define_expr_type(file_handler: &mut File, field_list: &Vec<&str>) {
        for t in field_list {
            let (struct_name, _) = t.split_once(":").unwrap();

            let _ = file_handler.write(
                ["    ", struct_name, "Expr(", struct_name, "),\n"]
                    .concat()
                    .as_bytes(),
            );
        }
        let _ = file_handler.write("}\n\n".as_bytes());
    }
    fn define_type(file_handler: &mut File, struct_name: &str, field_list: &str) {
        let _ = file_handler.write(
            ["#[derive(Debug)]\n", "pub struct", " ", struct_name, " {\n"]
                .concat()
                .as_bytes(),
        );

        let fields = field_list.split(", ");

        for field in fields {
            let field_split: Vec<&str> = field.split(" ").collect();
            let field_name = field_split[1];
            let field_type = field_split[0];
            let _ = file_handler.write(
                ["    ", "pub ", field_name, ": ", field_type, ",\n"]
                    .concat()
                    .as_bytes(),
            );
        }

        let _ = file_handler.write(b"}\n");
    }
}

fn main() {
    let main = Main {};
    main.main();
}
