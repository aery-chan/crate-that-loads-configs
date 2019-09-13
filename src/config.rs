#[path = "./format.rs"]
mod format;

use std::path::Path;
use std::io::Error;
use std::fs;

pub struct Config<'a, Content> {
    file_path: &'a Path,
    format: Box<dyn format::Format<Content>>,
    defaults: Option<Content>,
    content: Option<Content>
}

impl<'a, Content> Config<'a, Content> {

    pub fn new(file_path: &'a Path, format: Box<dyn format::Format<Content>>) -> Self {
        Self {
            file_path: file_path,
            format: format,
            defaults: None,
            content: None
        }
    }

    pub fn def(mut self, defaults: Content) -> Self {
        self.defaults = Some(defaults);
        self
    }

    pub fn read(mut self) -> Result<Self, Error> {
        let read_bytes: Vec<u8> = fs::read(self.file_path)?;
        self.content = Some(self.format.deserialize(read_bytes, &self.defaults));
        Ok(self)
    }

    pub fn write(mut self) -> Result<Self, Error> {
        let deserialized: Vec<u8> = self.format.serialize(&self.content);
        fs::write(self.file_path, deserialized)?;
        Ok(self)
    }
    
}