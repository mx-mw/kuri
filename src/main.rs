extern crate glob;

mod codegen;
mod discover_files;
mod file_rw;
mod nio;
mod read_config;

use codegen::codegen;
use discover_files::discover_files;
use file_rw::{read_blueprint, write_generated_file};
use nio::*;
use read_config::ConfigFile;
use std::env;

fn main() {
    let argv = env::args();
    let argc = argv.len();
    let args = argv.collect::<Vec<String>>();

    if argc == 4 && (args[1] == "g" || args[1] == "generate") {
        let config = ConfigFile::read();
        let input_directory = match config.project.blueprint_dir {
            None => "blueprints",
            Some(ref p) => p.as_str(),
        };
        let output_directory = match config.project.src_dir {
            None => "src",
            Some(ref p) => p.as_str(),
        };
        let mut generated: Vec<String> = vec![];
        for i in discover_files(input_directory.to_string()) {
            let bp_file: &str;
            let mut generate_path = String::new();
            let os_path_standard: char;
            if i.split('/').count() == 1 {
                os_path_standard = '\\';
                let vec = i.split('\\').collect::<Vec<&str>>();
                bp_file = match vec.last() {
                    None => "",
                    Some(f) => f,
                };
                if let Some((_, elements)) = vec.split_last() {
                    if let Some((_, first)) = elements.split_first() {
                        for j in first {
                            generate_path.push_str(j);
                            generate_path.push_str("\\");
                        }
                    }
                }
            } else {
                os_path_standard = '/';
                let vec = i.split('/').collect::<Vec<&str>>();
                bp_file = match vec.last() {
                    None => "",
                    Some(f) => f,
                };
                if let Some((_, elements)) = vec.split_last() {
                    if let Some((_, first)) = elements.split_first() {
                        for j in first {
                            generate_path.push_str(j);
                            generate_path.push_str("/");
                        }
                    }
                }
            }
            let bp_type = bp_file.split('.').collect::<Vec<&str>>()[0];
            let filetype = bp_file.split('.').collect::<Vec<&str>>()[1];
            if bp_type == args[2] {
                let source: String = read_blueprint(format!(
                    "{}{}{}{}.{}.kbp",
                    input_directory, os_path_standard, generate_path, bp_type, filetype
                ));
                if source != "".to_string() {
                    let out = codegen(source, config.clone());
                    match write_generated_file(
                        format!(
                            "{}{}{}{}.{}",
                            output_directory, os_path_standard, generate_path, args[3], filetype
                        ),
                        out,
                        format!("{}{}{}", output_directory, os_path_standard, generate_path),
                    ) {
                        Ok(()) => generated.push(format!("Generated {}.{}", args[3], filetype)),
                        Err(e) => red(format!(
                            "Error: {} {}\\{}{}.{}.kbp",
                            e, output_directory, generate_path, bp_type, filetype
                        )),
                    };
                }
            }
        }
        if generated.is_empty() {
            red(format!("Blueprint not found: {}", args[2]));
        } else {
            for i in generated {
                green(i);
            }
        }
    } else {
        red("Usage: kuri generate <blueprint> <module name>".to_string())
    }
}
