use crate::nio::*;

pub struct ConfigFileError {
    pub message: String
}

impl ConfigFileError {
    pub fn display(&self) {
        red(format!("{}", self.message));
    }
}