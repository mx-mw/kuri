use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct ConfigFile {
    pub template: Option<Template>,
    pub project: Project,
    pub meta: Meta,
    pub flags: Option<Flags>,
}
#[derive(Deserialize, Clone)]
pub struct Meta {
    pub kuri_version: String,
}

#[derive(Deserialize, Clone)]
pub struct Flags {
    pub module_name_rep: Option<String>,
    pub license_rep: Option<String>,
    pub version_rep: Option<String>,
    pub custom_flags: Option<Vec<CustomFlag>>,
}

#[derive(Deserialize, Clone)]
pub struct CustomFlag {
    pub name: String,
    pub replace_with: String,
}

#[derive(Deserialize, Clone)]
pub struct Template {
    pub language: String,
    pub variant: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub project_name: String,
    pub repo: Option<String>,
    pub license: Option<String>,
    pub version: Option<String>,
    pub blueprint_dir: Option<String>,
    pub src_dir: Option<String>,
}

impl ConfigFile {
    pub fn read() -> ConfigFile {
        let path_object = Path::new("kuri.toml");
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
        match toml::from_str(file_string.as_str()) {
            Err(e) => panic!("{}", e),
            Ok(deserialized) => deserialized,
        }
    }
}

pub fn get_directories<'a>(config: &'a ConfigFile) -> (String, String) {
    (
        match &config.project.blueprint_dir {
            None => "blueprints".to_string(),
            Some(p) => p.to_string(),
        },
        match &config.project.src_dir {
            None => "src".to_string(),
            Some(p) => p.to_string(),
        },
    )
}
