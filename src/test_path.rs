use std::sync::Mutex;
use std::path::{ PathBuf, Path };

lazy_static! {
    static ref ID: Mutex<u32> = Mutex::new(0);
    static ref DIR_GUARD: Mutex<()> = Mutex::new(());
}

pub struct TestPath(pub Box<Path>);

impl TestPath {

    pub fn new() -> Self {
        let id: &mut u32 = &mut *ID.lock().unwrap();

        if *id == u32::max_value() {
            panic!("Maximum amount of paths reached");
        }

        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(Path::new("."));
        path_buf.push(Path::new("tmp"));
        path_buf.push(id.to_string());

        *id += 1;

        Self(path_buf.into_boxed_path())
    }

}

impl Drop for TestPath {

    fn drop(&mut self) {

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
        let p1: &Box<Path> = &TestPath::new().0;
        let p2: &Box<Path> = &TestPath::new().0;
        assert_ne!(p1, p2);
    }

    fn test_path_thread() -> JoinHandle<TestPath> {
        thread::spawn(|| {
            TestPath::new()
        })
    }

    #[test]
    fn test_paths_unique_threads() {
        let p1: &Box<Path> = &test_path_thread().join().unwrap().0;
        let p2: &Box<Path> = &test_path_thread().join().unwrap().0;
        assert_ne!(p1, p2);
    }

}