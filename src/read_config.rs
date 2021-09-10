use serde::Deserialize;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
use crate::error::ConfigFileError;

/********************************************
*****Config File structs for TOML parser*****
********************************************/

#[derive(Deserialize, Clone)]
pub struct ConfigFile {
    pub project: Project,
    pub flags: Flags,
}

#[derive(Deserialize, Clone)]
pub struct Flags {
    pub flags: Vec<CustomFlag>,
}

#[derive(Deserialize, Clone)]
pub struct CustomFlag {
    pub name: String,
    pub source: String,
    pub replace_with: String,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub project_name: String,
    pub blueprint_dir: Option<String>,
    pub src_dir: Option<String>,
}

/*****************************
*****Read the config file*****
*****************************/

impl ConfigFile {
    pub fn read(input_text: Option<String>) -> Result<ConfigFile, ConfigFileError> {
        let mut config_text = String::new();
        if let Some(inp) = input_text {
            config_text = inp;
        } else {
            let path_object = Path::new("kuri.toml");
            let display = path_object.display();
            let mut file = match File::open(&path_object) {
                Err(reason) => panic!("Couldn't open file {}: {}", display, reason),
                Ok(file) => file,
            };
            match file.read_to_string(&mut config_text) {
                Err(reason) => panic!("Couldn't read {}: {}", display, reason),
                Ok(_) => file,
            };
        }

        match toml::from_str(config_text.as_str()) {
            Err(e) => Err(ConfigFileError{ message: format!("Error in kuri.toml: {}", e)}),
            Ok(deserialized) => Ok(deserialized),
        }
    }
}



/**********************************
*****PartialEq Implementations*****
**********************************/

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.project_name == other.project_name
            && self.blueprint_dir == other.blueprint_dir
            && self.src_dir == other.src_dir
    }
}

impl PartialEq for CustomFlag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.replace_with == other.replace_with
    }
}

impl PartialEq for ConfigFile {
    fn eq(&self, other: &Self) -> bool {
            self.project == other.project
    }
}

/******************************
*****Debug Implementations*****
******************************/


impl fmt::Debug for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project")
            .field("blueprint_dir", &self.blueprint_dir)
            .field("project_name", &self.project_name)
            .field("src_dir", &self.src_dir)
            .finish()
    }
}


impl fmt::Debug for CustomFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomFlag")
            .field("name", &self.name)
            .field("replace_with", &self.replace_with)
            .field("source", &self.source)
            .finish()
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
            .field("flags", &self.flags)
            .finish()
    }
}

impl fmt::Debug for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigFile")
            .field("project", &self.project)
            .field("flags", &self.flags)
            .finish()
    }

}

/*********************************************
*****Get the input and output directories*****
*********************************************/

pub fn get_directories(config: &'_ ConfigFile) -> (String, String) {
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
