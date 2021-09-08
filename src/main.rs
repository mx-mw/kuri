extern crate glob;

mod codegen;
mod discover_files;
mod file_rw;
mod nio;
mod read_config;
mod test;
mod error;

use discover_files::discover_files_loop;
use file_rw::{get_wd, init_new_config};
use nio::*;
use read_config::{get_directories, ConfigFile};
use std::env;
use clap::{App, load_yaml};

/**********************
*****Main Function*****
**********************/

fn main() {
    /******************************
    *****Load clap yaml config*****
    ******************************/
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    /******************************************
    *****Get args to appropriate variables*****
    ******************************************/
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    /*************************
    *****Generate Command*****
    *************************/

    if let Some(ref matches) = matches.subcommand_matches("generate") {
        if matches.is_present("path") {
            let config = match ConfigFile::read(None) {
                Ok(config) => config,
                Err(e) => {
                    println!("{}", e.message);
                    return;
                }
            };
            let (input_directory, output_directory) = get_directories(&config);
    
            let mut generated: Vec<String> = vec![];
            discover_files_loop(
                input_directory.as_str(),
                output_directory.as_str(),
                &args,
                &config,
                &mut generated,
            );
    
            if generated.is_empty() {
                red(format!("No blueprints found for {}", args[2]));
            } else {
                for i in generated {
                    green(i);
                }
            }
        }
        

    /***********************************
    *****Initialize project command*****
    ***********************************/
    } else if argc == 2 && args[1] == "init" {
        match init_new_config() {
            Ok(_) => green(format!("Project initalised in {}", get_wd())),
            Err(_) => red(format!("Error: project already initalised in {}", get_wd())),
        }

    /********************************************
    *****No command found, show proper usage*****
    *********************************************/
    } else {
        println!("Usage:");
        red("kuri generate <blueprint> <module name>".to_string());
        red("kuri init".to_string());
    }
}
