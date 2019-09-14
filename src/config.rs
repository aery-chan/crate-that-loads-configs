use std::path::Path;
use std::io::Error;
use std::fs;

use crate::format;

pub struct Config<'a, Format: format::Format> {
    file_path: &'a Path,
    format: Box<Format>,
    defaults: Option<Format::Content>,
    content: Option<Format::Content>
}

impl<'a, Format: format::Format> Config<'a, Format> {

    pub fn new(file_path: &'a Path, format: Box<Format>) -> Self {
        Self {
            file_path: file_path,
            format: format,
            defaults: None,
            content: None
        }
    }

    pub fn def(mut self, defaults: Format::Content) -> Self {
        self.defaults = Some(defaults);
        self
    }

    pub fn read(mut self) -> Result<Self, Error> {
        let read_bytes: Vec<u8> = fs::read(self.file_path)?;
        let defaults: Option<&Format::Content> = match &self.defaults {
            Some(__defaults) => Some(*&__defaults),
            None => None
        };

        self.content = Some(self.format.deserialize(read_bytes, defaults));
        
        Ok(self)
    }

    pub fn write(mut self) -> Result<Self, Error> {
        let content: Option<&Format::Content> = match &self.content {
            Some(content) => Some(*&content),
            None => None
        };
        let deserialized: Vec<u8> = self.format.serialize(content);

        fs::write(self.file_path, deserialized)?;

        Ok(self)
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::formats::string_format::StringFormat;

    #[test]
    fn new_config() {
        Config::new(Path::new("./test.txt"), Box::new(StringFormat::new()));
    }

    /*
    #[test]
    fn config_read() {
        let p: &Path = Path::new("./test.txt");
        fs::write(p, String::from("Hello, world!").as_bytes());
    }
    */
}