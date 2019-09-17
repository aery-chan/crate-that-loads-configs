use std::path::{ Path, PathBuf };

use crate::format;
use crate::config::Config;

pub struct ConfigDirectory<'a, Format: format::Format> {
    pub path: &'a Path,
    configs: Vec<Config<'a, Format>>
}

impl<'a, Format: format::Format> ConfigDirectory<'a, Format> {

    pub fn new(path: &'a Path) -> Self {
        Self {
            path,
            configs: Vec::new()
        }
    }

    pub fn conf(mut self, mut config: Config<'a, Format>) -> Self {
        let path: &Path = config.path;

        if let Some(_) = path.ancestors().next() {
            panic!("Config path may not have any ancestors");
        }

        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(self.path);
        path_buf.push(path);

        config.path = &path_buf.into_boxed_path();
   
        self.configs.push(config);
        self
    }

}