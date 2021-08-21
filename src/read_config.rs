use serde::Deserialize;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

/********************************************
*****Config File structs for TOML parser*****
********************************************/

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
    pub source: String,
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

/*****************************
*****Read the config file*****
*****************************/

impl ConfigFile {
    pub fn read(input_text: Option<String>) -> ConfigFile {
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
            Err(e) => panic!("{}", e),
            Ok(deserialized) => deserialized,
        }
    }
}



/**********************************
*****PartialEq Implementations*****
**********************************/

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.project_name == other.project_name
            && self.license == other.license
            && self.version == other.version
            && self.blueprint_dir == other.blueprint_dir
            && self.src_dir == other.src_dir
    }
}

impl PartialEq for CustomFlag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.replace_with == other.replace_with
    }
}

impl PartialEq for Flags {
    fn eq(&self, other: &Self) -> bool {
        self.custom_flags == other.custom_flags
    }
}

impl PartialEq for Meta {
    fn eq(&self, other: &Self) -> bool {
        self.kuri_version == other.kuri_version   
    }
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.language == other.language && self.variant == other.variant
    }
}

impl PartialEq for ConfigFile {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
            && self.project == other.project
            && self.meta == other.meta
            && self.template == other.template
    }
}

/******************************
*****Debug Implementations*****
******************************/

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template")
            .field("language", &self.language)
            .field("variant", &self.variant)
            .finish()
    }
}

impl fmt::Debug for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project")
            .field("blueprint_dir", &self.blueprint_dir)
            .field("license", &self.license)
            .field("project_name", &self.project_name)
            .field("repo", &self.repo)
            .field("src_dir", &self.src_dir)
            .field("version", &self.version)
            .finish()
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Meta")
            .field("kuri_version", &self.kuri_version)
            .finish()
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
            .field("kuri_version", &self.custom_flags)
            .field("kuri_version", &self.license_rep)
            .field("kuri_version", &self.module_name_rep)
            .field("kuri_version", &self.version_rep)
            .finish()
    }
}

impl fmt::Debug for CustomFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomFlag")
            .field("name", &self.name)
            .field("kuri_version", &self.replace_with)
            .finish()
    }
}

impl fmt::Debug for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigFile")
            .field("template", &self.template)
            .field("project", &self.project)
            .field("meta", &self.meta)
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
