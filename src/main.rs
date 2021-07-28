mod file_io;
mod codegen;

use std::env;
use file_io::{read_blueprint, write_generated_file};
use codegen::{codegen, CodegenOptions};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len(); 

    if argc == 1 {
        let source: String = read_blueprint(String::from("blueprints/test.kbp")).clone();
        let opts = CodegenOptions::new("epical first test", Some("Wee Woo MIT license"), Some("0.0.1"));
        
        let out = codegen(source, opts);
        match write_generated_file("out.test", out) {
            Ok(()) => println!("okei dokei"),
            Err(e) => panic!("{}", e)
        }
    }

}
