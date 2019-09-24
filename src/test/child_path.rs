use std::path::{ Path, PathBuf };

pub trait ChildPath {

    fn child_path(&self, config_name: &str) -> Box<Path>;

}

pub fn child_path(path: &Path, config_name: &str) -> Box<Path> {
    let mut path_buf: PathBuf = PathBuf::new();

    path_buf.push(&path);
    path_buf.push(Path::new(config_name));

    path_buf.into_boxed_path()
}