use std::process;
use crate::nio::*;

pub struct ConfigFileError {
    pub message: String
}

impl ConfigFileError {
    pub fn display(&self) {
        println!("{}", self.message);
    }
}