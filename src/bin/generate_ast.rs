use std::io::Write;
use std::{io, process};

pub struct Main {
    args: Vec<String>,
}

impl Main {
    //
    fn main(self) {
        let args_length = self.args.len();
        if args_length != 1 {
            println!("Usage: generate_ast <output dir>");
            process::exit(64);
        }
        let output_dir: &_ = &self.args[2];
        Self::define_ast(
            output_dir,
            "Expr",
            Vec::from([
                "Binary: Expr left, Token operator, Expr right",
                "Grouping: Expr expression",
                "Literal: Struct  value",
                "Unary: Token operator, Expr right",
            ]),
        )
    }
    fn define_ast(output_dir: &String, basename: &str, types: Vec<&str>) {
        let path = [output_dir, "/", basename, ".rs"].concat();
        let mut std_out_handler: io::Stdout = io::stdout();

        std_out_handler.write_all(["mod Expr", basename, "{"].concat().as_bytes());
        std_out_handler.write_all(b"}");


        for t in types {
            let struct_name_and_fields = t.split_once(":").unwrap();
            let structname = struct_name_and_fields.0.trim();
            let fields = struct_name_and_fields.1.trim();
        }
    }

    fn define_type(basename: &str, structname: &str, fieldList: &str) {
        let mut std_out_handler: io::Stdout = io::stdout();

        let _ = std_out_handler.write_all(
            ["struct", structname, "extends", basename]
                .concat()
                .as_bytes(),
        );
        let _ = std_out_handler.write_all(["     ", "(", fieldList, ")", "{"].concat().as_bytes());
        let fields = fieldList.split(", ");

        for field in fields {
            let _ = std_out_handler.write_all(["     final", field, ";"].concat().as_bytes());
        }
        let _ = std_out_handler.write_all("  }".as_bytes());
    }
}
