use std::path::Path;
use std::fs;

pub struct TestFile<'a> {
    path: &'a Path
}

impl<'a> TestFile<'a> {

    pub fn new(path: &'a Path) -> Self {
        Self { path: path }
    }

    pub fn read(&self) -> String {
        String::from_utf8(fs::read(self.path).unwrap()).unwrap()
    }

    pub fn write(&self, content: &String) {
        fs::write(self.path, content.as_bytes()).unwrap();
    }

}

impl<'a> Drop for TestFile<'a> {

    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_file(self.path).unwrap();
        }
    }

}