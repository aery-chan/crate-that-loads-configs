use std::path::Path;
use std::fs;
use std::io::Error;

use crate::format;
use crate::config_file::ConfigFile;
use crate::config_directory::ConfigDirectory;

pub enum Config<Format: format::Format + Sized + Clone> {
    File(ConfigFile<Format>),
    Directory(ConfigDirectory<Format>)
}

/// Ensures that directory and it's ancestors exists
pub(crate) fn ensure(path: &Path) -> Result<(), Error> {
    if !path.is_dir() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test::test_path::TestPath;
    use crate::test::child_path::ChildPath;

    #[test]
    fn ensure_self() {
        let p: &Path = &TestPath::new().path;
        
        ensure(&p).unwrap();
        assert!(p.is_dir());
    }

    #[test]
    fn ensure_ancestors() {
        let tp: TestPath = TestPath::new();
        let p: &Path = &tp.child_path("test");

        ensure(&p).unwrap();
        assert!(p.is_dir());
    }

}