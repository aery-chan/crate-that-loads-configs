use std::path::{ Path, PathBuf };

use crate::format;
use crate::config_file::ConfigFile;
use crate::config::Config;

pub struct ConfigDirectory<Format: format::Format> {
    pub path: Box<Path>,
    pub configs: Vec<Config<Format>>
}

impl<Format: format::Format> ConfigDirectory<Format> {

    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path(),
            configs: Vec::new()
        }
    }

    fn dir_path(&self, path: Box<Path>) -> Box<Path> {
        if path.ancestors().next().is_some() {
            panic!("Config path which is to be added to a directory may not have any ancestors");
        }

        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(&self.path);
        path_buf.push(path);

        path_buf.into_boxed_path()
    }

    pub fn file(mut self, mut config_file: ConfigFile<Format>) -> Self {
        config_file.path = self.dir_path(config_file.path);
        self.configs.push(Config::File(config_file));
        self
    }

    pub fn dir(mut self, mut config_dir: ConfigDirectory<Format>) -> Self {
        config_dir.path = self.dir_path(config_dir.path);
        self.configs.push(Config::Directory(config_dir));
        self
    }

}