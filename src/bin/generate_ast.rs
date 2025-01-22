use std::fs::{DirBuilder, File};
use std::io::Write;
use std::{env, process};

pub struct Main {
    // args: Vec<String>,
}

impl Main {
    fn main(self) {
        // let args_length = self.args.len();
        // if args_length == 0 {
        //     println!("Usage: generate_ast <output dir>");
        //     process::exit(64);
        // }
        Self::define_ast(
            "expr",
            Vec::from([
                "Binary: String left, Token<String> operator, String right",
                "Grouping: String expression",
                "Literal<T>: Option<T> value",
                "Unary: Token<String> operator, String right",
            ]),
        )
    }
    fn define_ast(basename: &str, types: Vec<&str>) {
        let path = ["src", "/", basename, ".rs"].concat();

        let mut file_handler = File::create(&path).unwrap();

        let _ = file_handler.write(
            [
                "pub mod",
                " ",
                basename,
                " {\n",
                "use crate::token_type::Token;\n\n",
            ]
            .concat()
            .as_bytes(),
        );

        let _ = file_handler.write(
            [
                "trait ExpressionBehaviors {\n",
                "  fn interpret(&self) -> ();\n",
                "  fn resolve(&self) -> ();\n",
                "  fn analyze(&self) -> ();\n",
                "}",
                "\n\n",
            ]
            .concat()
            .as_bytes(),
        );

        for t in &types {
            let (struct_name, fields) = t.split_once(":").unwrap();
            Self::define_type(&mut file_handler, struct_name.trim(), fields.trim());
        }
        for t in &types {
            let (struct_name, _) = t.split_once(":").unwrap();
            Self::define_impls(&mut file_handler, struct_name.trim());
        }
        let _ = file_handler.write(["}"].concat().as_bytes());
    }

    fn define_type(file_handler: &mut File, struct_name: &str, field_list: &str) {
        let _ = file_handler.write(["pub struct", " ", struct_name, " {\n"].concat().as_bytes());

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
    fn define_impls(file_handler: &mut File, struct_name: &str) {
        let _ = file_handler.write(
            [
                "impl ExpressionBehaviors for ",
                struct_name,
                " { \n",
                "  fn interpret(&self) -> () {
                        ()
                    }\n",
                "  fn resolve(&self) -> () {
                        ()
                    }\n",
                "  fn analyze(&self) -> () {
                        ()
                    }\n",
                "}",
                "\n",
            ]
            .concat()
            .as_bytes(),
        );
    }
}

fn main() {
    let main = Main {
        // args: env::args().collect(),
    };
    main.main();
}
