use std::fs::{DirBuilder, File};
use std::io::Write;
use std::{env, process};

pub struct Main {
    args: Vec<String>,
}

impl Main {
    fn main(self) {
        let args_length = self.args.len();
        if args_length == 0 {
            println!("Usage: generate_ast <output dir>");
            process::exit(64);
        }
        let output_dir: &_ = &self.args[1];
        let path_to_output_dir = ["src/", "bin/", "generated/", output_dir].concat();
        let _ = DirBuilder::new()
            .recursive(true)
            .create(&path_to_output_dir);
        Self::define_ast(
            path_to_output_dir,
            "Expr",
            Vec::from([
                "Binary: String left, Token operator, String right",
                "Grouping: String expression",
                "Literal<T>: Option<T> value",
                "Unary: Token operator, String right",
            ]),
        )
    }
    fn define_ast(path_to_output_dir: String, basename: &str, types: Vec<&str>) {
        let path = [
            path_to_output_dir,
            "/".to_string(),
            basename.to_string(),
            ".rs".to_string(),
        ]
        .concat();

        let mut file_handler = File::create(&path).unwrap();

        let _ = file_handler.write("use crate::token_type::Token;\n\n".as_bytes());

        let _ = file_handler.write(["mod", " ", basename, "{"].concat().as_bytes());

        for t in types {
            let (struct_name, fields) = t.split_once(":").unwrap();
            Self::define_type(
                &mut file_handler,
                basename,
                struct_name.trim(),
                fields.trim(),
            );
        }
        let _ = file_handler.write(["}"].concat().as_bytes());
    }

    fn define_type(file_handler: &mut File, _: &str, struct_name: &str, field_list: &str) {
        let _ = file_handler.write(["struct", " ", struct_name, "{"].concat().as_bytes());

        let fields = field_list.split(", ");

        for field in fields {
            let field_split: Vec<&str> = field.split(" ").collect();
            let field_name = field_split[1];
            let field_type = field_split[0];
            let _ = file_handler.write(
                ["    ", field_name, ": ", field_type, ","]
                    .concat()
                    .as_bytes(),
            );
        }
        let _ = file_handler.write(b"}");
    }
}

fn main() {
    let main = Main {
        args: env::args().collect(),
    };
    main.main();
}
