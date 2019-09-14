use std::path::Path;
use std::io::Error;
use std::fs;

use crate::format;

pub struct Config<'a, Format: format::Format + Sized> {
    file_path: &'a Path,
    format: Format,
    defaults: Option<Format::Defaults>,
    content: Option<Format::Content>
}

impl<'a, Format: format::Format + Sized> Config<'a, Format> {

    pub fn new(file_path: &'a Path, format: Format) -> Self {
        Self {
            file_path,
            format,
            defaults: None,
            content: None
        }
    }

    pub fn def(&mut self, defaults: Format::Defaults) -> &mut Self {
        self.defaults = Some(defaults);
        self
    }

    pub fn read(&mut self) -> Result<&mut Self, Error> {
        let read_bytes: Vec<u8> = if self.file_path.exists() {
            fs::read(self.file_path)?
        } else {
            vec![]
        };
        let defaults: Option<&Format::Defaults> = match &self.defaults {
            Some(__defaults) => Some(__defaults),
            None => None
        };

        self.content = Some(self.format.deserialize(read_bytes, defaults));
        
        Ok(self)
    }

    pub fn write(&mut self) -> Result<&mut Self, Error> {
        let content: Option<&Format::Content> = match &self.content {
            Some(content) => Some(content),
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
    use crate::test::test_path::TestPath;
    use crate::test::test_file::TestFile;
    use crate::formats::string_format::StringFormat;

    #[test]
    fn new_config() {
        Config::new(Path::new("./test.txt"), StringFormat::new());
    }

    #[test]
    fn config_read() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: Config<StringFormat> = Config::new(p, StringFormat::new());

        f.write(&s);
        c.read().unwrap();

        assert_eq!(c.content.unwrap().as_str(), s);
    }

    #[test]
    fn config_write() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: Config<StringFormat> = Config::new(p, StringFormat::new());

        c.content = Some(s.clone());
        c.write().unwrap();

        assert_eq!(f.read(), s);
    }
    
    #[test]
    fn config_defaults() {
        let p: &Path = &TestPath::new().path;
        let _f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: Config<StringFormat> = Config::new(p, StringFormat::new());

        c.def(s.clone());
        c.read().unwrap();

        assert_eq!(c.content.unwrap(), s);
    }

}