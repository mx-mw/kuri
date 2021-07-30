use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::path::Path;

pub fn read_blueprint(path_string: String) -> String {
    let path_object = Path::new(&path_string);
    let display = path_object.display();
    let mut file = match File::open(&path_object) {
        Err(_) => return "".to_string(),
        Ok(file) => file,
    };
    println!("display: {}", display);

    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(reason) => panic!("Couldn't read {}: {}", display, reason),
        Ok(_) => file,
    };
    file_string
}

pub fn write_generated_file(
    path: String,
    data: String,
    folder_path: String,
) -> std::io::Result<()> {
    if Path::new(folder_path.as_str()).exists() {
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        return Ok(())
    } else {
        create_dir(folder_path.as_str())?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        return Ok(())
    }
}

pub fn read_license_file(path: String) -> String {
    let path_object = Path::new(&path);
    let display = path_object.display();
    let mut file = match File::open(&path_object) {
        Err(reason) => panic!("Couldn't open file {}: {}", display, reason),
        Ok(file) => file,
    };

    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(reason) => panic!("Couldn't read {}: {}", display, reason),
        Ok(_) => file,
    };
    file_string + "\n"
}
