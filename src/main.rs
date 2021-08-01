extern crate glob;

mod codegen;
mod discover_files;
mod file_rw;
mod nio;
mod read_config;

use discover_files::discover_files_loop;
use file_rw::{get_wd, init_new_config};
use nio::*;
use read_config::{get_directories, ConfigFile};
use std::env;

/**********************
*****Main Function*****
**********************/

fn main() {
    /******************************************
    *****Get Args to appropriate variables*****
    ******************************************/
    let argv = env::args();
    let argc = argv.len();
    let args = argv.collect::<Vec<String>>();

    /*************************
    *****Generate Command*****
    *************************/

    if argc >= 4 && (args[1] == "g" || args[1] == "generate") {
        let config = ConfigFile::read(None);
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

/**************
*****Tests*****
***************/

#[cfg(test)]
mod tests {
    use crate::file_rw::{get_os_path_standard, read_file};
    use crate::discover_files::discover_files;
    use crate::codegen::codegen;
    use crate::read_config::{ConfigFile, Meta, Project};

    use indoc::indoc;
    
    /************************************************************
    *****Test that the configuration file generates properly*****
    ************************************************************/
    
    #[test]
    fn config_test() {
        let conf = indoc! {"
        [project]
        project_name=\"TestProject\"
        
        [meta]
        kuri_version=\"1.0\""}.to_string();

        let project = Project {
            project_name: "TestProject".to_string(),
            repo: None,
            license: None,
            version: None,
            blueprint_dir: None,
            src_dir: None,
        };

        let meta = Meta {
            kuri_version: "1.0".to_string(),
        };

        let conf_struct = ConfigFile {
            flags: None,
            meta: meta,
            project: project,
            template: None,
        };

        assert_eq!(conf_struct, ConfigFile::read(Some(conf)))
    }

    #[test]
    fn codegen_test() {
        let config_string = indoc! {"
        [project]
        project_name=\"TestProject\"
        
        [meta]
        kuri_version=\"1.0\""}.to_string();
        let config = ConfigFile::read(Some(config_string));
        assert_eq!(codegen("%!%ModuleName%!%".to_string(), "CodegenTest".to_string(), config, &["".to_string()]), "CodegenTest".to_string());
    }

    #[test]
    fn file_dicovery_test() {
        assert_eq!(vec![format!("test{0}file_discovery{0}fd.test.kbp", get_os_path_standard())], discover_files(format!("test{0}file_discovery", get_os_path_standard())));
    }

    #[test]
    fn read_file_test() {
        // test if the read file function actually works
        assert_eq!(read_file(format!("test{0}read_file{0}fr0.test", get_os_path_standard())), "Test\n");
        assert_eq!(read_file(format!("test{0}read_file{0}fr1.test", get_os_path_standard())), "Test Numero Uno\n");
        assert_eq!(read_file(format!("test{0}read_file{0}fr2.test", get_os_path_standard())), "Test Numero Duo\n");

        // make sure newlines are added
        assert_ne!(read_file(format!("test{0}read_file{0}fr0.test", get_os_path_standard())), "Test");
        assert_ne!(read_file(format!("test{0}read_file{0}fr1.test", get_os_path_standard())), "Test Numero Uno");
        assert_ne!(read_file(format!("test{0}read_file{0}fr2.test", get_os_path_standard())), "Test Numero Duo");
        }

}
