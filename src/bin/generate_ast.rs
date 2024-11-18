use std::io::Write;
use std::{env, process};
use std::fs::File;

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
        Self::define_ast(
            output_dir,
            "Expr",
            Vec::from([
                "Binary: Expr left, Token operator, Expr right",
                "Grouping: Expr expression",
                "Literal: Struct value",
                "Unary: Token operator, Expr right",
            ]),
        )
    }
    fn define_ast(output_dir: &String, basename: &str, types: Vec<&str>) {
        let path: String = [basename, ".rs"].concat();
        let mut file_handler = File::create(&path).unwrap();

        let _ = file_handler.write(["mod", " ", basename, "{"].concat().as_bytes());
        for t in types {
            let struct_name_and_fields = t.split_once(":").unwrap();
            let struct_name = struct_name_and_fields.0.trim();
            let fields = struct_name_and_fields.1.trim();
            Self::define_type(&mut file_handler, basename, struct_name, fields);
        }
        let _ = file_handler.write(["}"].concat().as_bytes());
    }

    fn define_type(file_handler: &mut File, _: &str, struct_name: &str, field_list: &str) {
        let _ = file_handler.write(["struct", " ", struct_name, "{"].concat().as_bytes());

        let fields = field_list.split(", ");

        // todo!(
        //     "is the struct to be outputted fully correct?"
        // );
        // todo!("all of this structure is a Java class in the book; here, I'm trying to work with a struct; try to imagine possible corner cases that I'll maybe have to face");
        // todo!("don't get stuck on little implementation details from this section; I have been dealing with this portion of the project for a while already");

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
