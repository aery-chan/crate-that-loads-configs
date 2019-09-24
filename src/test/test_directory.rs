use std::path::Path;
use std::fs;

use crate::test::child_path;
use child_path::ChildPath;

pub struct TestDirectory {
    pub path: Box<Path>
}

impl TestDirectory {

    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path()
        }
    }

    pub fn create(&self) {
        fs::create_dir(&self.path).unwrap();
    }

}

impl ChildPath for TestDirectory {

    fn child_path(&self, config_name: &str) -> Box<Path> {
        child_path::child_path(&self.path, config_name)
    }

}