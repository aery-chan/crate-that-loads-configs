use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use std::io::Error;
use std::fs;

use crate::format;
use crate::config_file::ConfigFile;
use crate::config::Config;

pub struct ConfigDirectory<Format: format::Format + Sized + Clone> {
    pub path: Box<Path>,
    pub configs: HashMap<String, Config<Format>>,

    format: Format
}

impl<Format: format::Format + Sized + Clone> ConfigDirectory<Format> {

    pub fn new(path: &Path, format: Format) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path(),
            configs: HashMap::new(),

            format
        }
    }

    fn dir_path(&self, path: &Path) -> Box<Path> {
        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(&self.path);
        path_buf.push(Path::new(path.file_name().unwrap()));

        path_buf.into_boxed_path()
    }

    fn config_name(&self, path: &Path) -> String {
        (*path.file_name().unwrap()).to_os_string().into_string().unwrap()
    }

    #[allow(clippy::ptr_arg)]
    fn has_config(&mut self, file_name: &String) -> Option<Config<Format>> {
        let path: &Path = Path::new(file_name);
        let mut found_key: Option<String> = None;

        for ( key, config ) in self.configs.iter() {
            let config_path: &Path;
            let dir_path: Box<Path>;

            match config {
                Config::File(config_file) => {
                    config_path = &config_file.path;
                    dir_path = self.dir_path(path);
                },
                Config::Directory(config_dir) => {
                    config_path = &config_dir.path;
                    dir_path = self.dir_path(path);
                }
            }

            if config_path == &*dir_path {
                found_key = Some(key.clone());
                break;
            }
        }

        match found_key {
            Some(key) => {
                self.configs.remove(&key)
            },
            None => None
        }
    }

    pub fn file(mut self, mut config_file: ConfigFile<Format>) -> Self {
        config_file.path = self.dir_path(&config_file.path);
        // TODO: Copy old file name instead of calling self.config_name
        self.configs.insert(self.config_name(&config_file.path), Config::File(config_file));
        self
    }

    pub fn dir(mut self, mut config_dir: ConfigDirectory<Format>) -> Self {
        config_dir.path = self.dir_path(&config_dir.path);
        self.configs.insert(self.config_name(&config_dir.path), Config::Directory(config_dir));
        self
    }

    pub fn read(mut self) -> Result<Self, Error> {
        for entry in fs::read_dir(&self.path)? {
            let entry: fs::DirEntry = entry?;
            let config_name: String = entry.file_name().into_string().unwrap();

            match self.has_config(&(&entry).file_name().into_string().unwrap()) {
                Some(config) => {
                    match config {
                        Config::File(config_file) => {
                            self.configs.insert(config_name, Config::File(config_file.read()?));
                        },
                        Config::Directory(config_dir) => {
                            self.configs.insert(config_name, Config::Directory(config_dir.read()?));
                        }
                    }
                },
                None => {
                    let file_type: fs::FileType = (&entry).file_type().unwrap();

                    if file_type.is_file() {
                        let config_file: ConfigFile<Format> =
                            ConfigFile::new(&self.dir_path(Path::new(&config_name)), self.format.clone())
                                .read()
                                .unwrap();
                        self.configs.insert(config_name, Config::File(config_file));
                    } else if file_type.is_dir() {
                        let config_dir: ConfigDirectory<Format> =
                            ConfigDirectory::new(&self.dir_path(Path::new(&config_name)), self.format.clone())
                                .read()
                                .unwrap();
                        self.configs.insert(config_name, Config::Directory(config_dir));
                    }
                }
            }
        }
        
        Ok(self)
    }

}