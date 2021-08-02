use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::path::Path;
use indoc::indoc;
use std::env;

/******************************
*****Read a blueprint file*****
******************************/

pub fn read_file(path_string: &'_ String) -> String {
    let path_object = Path::new(&path_string);
    let display = path_object.display();
    let mut file = match File::open(&path_object) {
        Err(_) => return "".to_string(),
        Ok(file) => file,
    };

    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(reason) => panic!("Couldn't read {}: {}", display, reason),
        Ok(_) => file,
    };
    file_string + "\n"
}

/************************************
*****Write a generated blueprint*****
************************************/

pub fn write_generated_file(
    path: String,
    data: String,
    folder_path: String,
) -> std::io::Result<()> {
    if Path::new(folder_path.as_str()).exists() {
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    } else {
        create_dir(folder_path.as_str())?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

/*************************************
*****Initialize a new config file*****
*************************************/

pub fn init_new_config() -> std::io::Result<()> {    
    let conf = indoc! {"
    [project]
    project_name=\"Project\"
    
    [meta]
    kuri_version=\"1.0\""};

    if !Path::new("kuri.toml").exists() {
        let mut file = File::create("kuri.toml")?;
        file.write_all(conf.as_bytes())?;
        return Ok(())
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, ""))
    
}

/**********************************
*****Get the working directory*****
**********************************/

pub fn get_wd() -> String {
    match std::env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => panic!("{}", e)
    }
}

/*********************************
*****Get the OS path standard*****
*********************************/

pub fn get_os_path_standard() -> char {
    if env::consts::OS == "windows" {
        '\\'
    } else {
        '/'
    }
}