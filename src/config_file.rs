use std::path::Path;
use std::io::Error;
use std::fs;

use crate::format;

pub struct ConfigFileOpts {
    write_if_defaulted: bool
}

impl Default for ConfigFileOpts {

    fn default() -> Self {
        ConfigFileOpts {
            write_if_defaulted: false
        }
    }
    
}

pub struct ConfigFile<Format: format::Format + Sized + Clone> {
    pub path: Box<Path>,
    pub options: ConfigFileOpts,
    pub content: Option<Format::Content>,
    pub defaulted: bool,

    format: Format,
    defaults: Option<Format::Defaults>
}

impl<Format: format::Format + Sized + Clone> ConfigFile<Format> {

    pub fn new(path: &Path, format: Format) -> Self {
        ConfigFile {
            path: path.to_path_buf().into_boxed_path(),
            options: ConfigFileOpts::default(),
            content: None,
            defaulted: false,

            format,
            defaults: None
        }
    }

    pub fn def(mut self, defaults: Format::Defaults) -> Self {
        self.defaults = Some(defaults);
        self
    }

    pub fn opt(mut self, options: ConfigFileOpts) -> Self {
        self.options = options;
        self
    }

    pub fn read(mut self) -> Result<Self, Error> {
        let bytes: Vec<u8> = if self.path.exists() {
            fs::read(&self.path)?
        } else {
            Vec::new()
        };
        let defaults: Option<&Format::Defaults> = match &self.defaults {
            Some(__defaults) => Some(__defaults),
            None => None
        };
        let deserialized: format::Deserialized<Format::Content> = self.format.deserialize(bytes, defaults);

        self.content = Some(deserialized.0);
        self.defaulted = deserialized.1;

        if self.defaulted && self.options.write_if_defaulted {
            self = self.write()?;
        }
        
        Ok(self)
    }

    pub fn write(mut self) -> Result<Self, Error> {
        let content: Option<&Format::Content> = match &self.content {
            Some(content) => Some(content),
            None => None
        };
        let deserialized: Vec<u8> = self.format.serialize(content);

        fs::write(&self.path, deserialized)?;

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
        ConfigFile::new(Path::new("test.txt"), StringFormat::new());
    }

    #[test]
    fn read() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: ConfigFile<StringFormat> = ConfigFile::new(p, StringFormat::new());

        f.write(&s);
        c = c.read().unwrap();

        assert_eq!(c.content.unwrap().as_str(), s);
    }

    #[test]
    fn write() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: ConfigFile<StringFormat> = ConfigFile::new(p, StringFormat::new());

        c.content = Some(s.clone());
        c.write().unwrap();

        assert_eq!(f.read(), s);
    }
    
    #[test]
    fn defaults() {
        let p: &Path = &TestPath::new().path;
        let s: String = String::from("Hello, world!");

        let c: ConfigFile<StringFormat> = ConfigFile::new(p, StringFormat::new())
            .def(s.clone())
            .read()
            .unwrap();

        assert_eq!(c.content.unwrap(), s);
    }

    #[test]
    fn defaulted() {
        let p: &Path = &TestPath::new().path;
        let c: ConfigFile<StringFormat> = ConfigFile::new(p, StringFormat::new())
            .def(String::from("Hello, world!"))
            .read()
            .unwrap();
        
        assert_eq!(c.defaulted, true);
    }

    #[test]
    fn not_defaulted() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");
        let mut c: ConfigFile<StringFormat> = ConfigFile::new(p, StringFormat::new())
            .def(s.clone());

        f.write(&s);
        c = c.read().unwrap();

        assert_eq!(c.defaulted, false);
    }

    #[test]
    fn write_if_defaulted() {
        let p: &Path = &TestPath::new().path;
        let f: TestFile = TestFile::new(p);
        let s: String = String::from("Hello, world!");

        ConfigFile::new(p, StringFormat::new())
            .def(s.clone())
            .opt(ConfigFileOpts {
                write_if_defaulted: true
            })
            .read()
            .unwrap();

        assert_eq!(f.read(), s);
    }

}