use std::process;

pub struct Main {
    args: Vec<String>,
}

impl Main {
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
            Vec::from(["Binary: Expr left, Token operator, Expr right", "Grouping: Expr expression", "Literal: Object value", "Unary: Token operator, Expr right"])
        )
    }
    fn define_ast(output_dir: &String, basename: &str, types: Vec<&str>){}
}