use std::fs;
use std::sync::Mutex;
use std::path::{ Path, PathBuf };
use std::collections::HashSet;

lazy_static! {
    static ref ID: Mutex<u32> = Mutex::new(0);
    static ref DIR: Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
}

pub struct TestPath {
    id: u32,
    pub path: Box<Path>
}

impl TestPath {

    fn try_mkdir(dir: &&mut HashSet<u32>) {
        let mut dir_path_buf: PathBuf = PathBuf::new();
        let dir_path: &Path;

        dir_path_buf.push(Path::new("."));
        dir_path_buf.push(Path::new("tmp"));

        dir_path = dir_path_buf.as_path();

        if (**dir).is_empty() {
            fs::create_dir(dir_path).unwrap();
        }
    }

    pub fn new() -> Self {
        let id: &mut u32 = &mut *ID.lock().unwrap();

        if *id == u32::max_value() {
            panic!("Maximum amount of paths reached");
        }

        let dir: &mut HashSet<u32> = &mut *DIR.lock().unwrap(); // Set of ID:s currently using folder
        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(Path::new("."));
        path_buf.push(Path::new("tmp"));
        path_buf.push((*id).to_string());

        // Create test directiory if no one else is using it.
        // i.e: It doesn't exist, since if we're the last to use it, we remove it when we're dropped
        Self::try_mkdir(&dir);

        (*dir).insert(*id);

        let ret_id: u32 = *id;

        *id += 1;

        Self {
            id: ret_id,
            path: path_buf.into_boxed_path()
        }
    }

}

impl Drop for TestPath {

    fn drop(&mut self) {
        let mut dir_path_buf: PathBuf = PathBuf::new();
        let dir: &mut HashSet<u32> = &mut *DIR.lock().unwrap();

        (*dir).remove(&self.id);

        dir_path_buf.push(Path::new("."));
        dir_path_buf.push(Path::new("tmp"));

        // Remove test dir if we were the last to use it
        if (*dir).is_empty() {
            fs::remove_dir(dir_path_buf.as_path()).unwrap()
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;
    use thread::JoinHandle;

    #[test]
    fn new_test_path() {
        TestPath::new();
    }

    #[test]
    fn test_paths_unique() {
        let p1: &Box<Path> = &TestPath::new().path;
        let p2: &Box<Path> = &TestPath::new().path;
        assert_ne!(p1, p2);
    }

    fn test_path_thread() -> JoinHandle<TestPath> {
        thread::spawn(|| {
            TestPath::new()
        })
    }

    #[test]
    fn test_paths_unique_threads() {
        let p1: &Box<Path> = &test_path_thread().join().unwrap().path;
        let p2: &Box<Path> = &test_path_thread().join().unwrap().path;
        assert_ne!(p1, p2);
    }

}