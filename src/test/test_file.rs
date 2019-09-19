use std::path::Path;
use std::fs;

pub struct TestFile {
    pub path: Box<Path>
}

impl TestFile {

    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path()
        }
    }

    pub fn read(&self) -> String {
        String::from_utf8(fs::read(&self.path).unwrap()).unwrap()
    }

    #[allow(clippy::ptr_arg)]
    pub fn write(&self, content: &String) {
        fs::write(&self.path, content.as_bytes()).unwrap();
    }

}

impl Drop for TestFile {

    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_file(&self.path).unwrap();
        }
    }

}