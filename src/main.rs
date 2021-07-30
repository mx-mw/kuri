extern crate glob;

mod codegen;
mod discover_files;
mod file_rw;
mod nio;
mod read_config;

use discover_files::discover_files_loop;
use nio::*;
use file_rw::{init_new_config, get_wd};
use read_config::ConfigFile;
use std::env;

fn main() {
    let argv = env::args();
    let argc = argv.len();
    let args = argv.collect::<Vec<String>>();
    if argc >= 4 && (args[1] == "g" || args[1] == "generate") {
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
        discover_files_loop(
            input_directory,
            output_directory,
            &args,
            &config,
            &mut generated,
        );
        if generated.is_empty() {
            red(format!("Blueprint not found: {}", args[2]));
        } else {
            for i in generated {
                green(i);
            }
        }
    } else if argc == 2 && args[1] == "init" {
        match init_new_config() {
            Ok(_) => green(format!("Project initalised in {}", get_wd())),
            Err(_) => red(format!("Error: project already initalised in {}", get_wd()))
        }
    } else {
        red("Usage: kuri generate <blueprint> <module name>".to_string())
    }
}
