use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use std::io::Error;
use std::fs;

use crate::format;
use crate::config_file::ConfigFile;
use crate::config;
use config::Config;

pub struct ConfigDirOpts {
    pub write_if_defaulted: bool,
    pub read_new: bool,
    pub recursive: bool
}

impl Default for ConfigDirOpts {

    fn default() -> Self {
        Self {
            write_if_defaulted: false,
            read_new: false,
            recursive: false
        }
    }

}

pub struct ConfigDirectory<Format: format::Format + Sized + Clone> {
    pub path: Box<Path>,
    pub configs: HashMap<String, Config<Format>>,
    pub defaulted: bool,

    format: Format,
    options: ConfigDirOpts
}

impl<Format: format::Format + Sized + Clone> ConfigDirectory<Format> {

    pub fn new(path: &Path, format: Format) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path(),
            configs: HashMap::new(),
            defaulted: false,

            format,
            options: ConfigDirOpts::default()
        }
    }

    fn child_path(&self, path: &Path) -> Box<Path> {
        let mut path_buf: PathBuf = PathBuf::new();

        path_buf.push(&self.path);
        path_buf.push(Path::new(path.file_name().unwrap()));

        path_buf.into_boxed_path()
    }

    fn config_name(&self, path: &Path) -> String {
        (*path.file_name().unwrap()).to_os_string().into_string().unwrap()
    }

    pub fn file(mut self, mut config_file: ConfigFile<Format>) -> Self {
        config_file.path = self.child_path(&config_file.path);
        self.configs.insert(self.config_name(&config_file.path), Config::File(config_file));
        self
    }

    pub fn dir(mut self, mut config_dir: ConfigDirectory<Format>) -> Self {
        config_dir.path = self.child_path(&config_dir.path);
        self.configs.insert(self.config_name(&config_dir.path), Config::Directory(config_dir));
        self
    }

    #[allow(clippy::ptr_arg)]
    fn has_config(&self, path: &Path) -> bool {
        for config in self.configs.values() {
            let config_path: &Path;

            match config {
                Config::File(config_file) => {
                    config_path = &config_file.path;
                },
                Config::Directory(config_dir) => {
                    config_path = &config_dir.path;
                }
            }

            if config_path == &*path {
                return true;
            }
        }

        false
    }

    fn children(&self) -> Vec<String> {
        let mut children: Vec<String> = vec![];

        for ( key, config ) in self.configs.iter() {
            let config_path: &Path;

            match config {
                Config::File(config_file) => {
                    config_path = &config_file.path;
                },
                Config::Directory(config_dir) => {
                    config_path = &config_dir.path;
                }
            }

            if config_path.parent().unwrap() == &*self.path {
                children.push(key.clone());
            }
        }

        children
    }

    pub fn read(mut self) -> Result<Self, Error> {
        // We should only read new configs if read_new is enabled.
        // If we're supposed to read new configs, we just insert any new configs found in our directory
        // to be read in the next step bellow
        if self.options.read_new && self.path.is_dir() {
            for entry in fs::read_dir(&self.path)? {
                let entry: fs::DirEntry = entry?;
                let config_name: String = entry.file_name().into_string().unwrap();
                let config_path: Box<Path> = self.child_path(Path::new(&config_name));

                if !self.has_config(&config_path) {
                    let file_type: fs::FileType = entry.file_type()?;

                    if file_type.is_file() {
                        self.configs.insert(config_name, Config::File(ConfigFile::new(&config_path, self.format.clone())));
                    } else if file_type.is_dir() {
                        self.configs.insert(config_name, Config::Directory(ConfigDirectory::new(&config_path, self.format.clone())));
                    }
                }
            }
        }

        for key in self.children() {
            // We should only read directory contents if recursive is enabled.
            // Since the read functions require that we retrieve ownership,
            // we want to be sure beforehand if we're supposed to read the config,
            // which we figure out here
            let config: &Config<Format> = self.configs.get(&key).unwrap();
            let should_read: bool = if let Config::Directory(_) = config {
                self.options.recursive
            } else {
                true
            };

            if should_read {
                let config: Config<Format> = self.configs.remove(&key).unwrap();
            //                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            //                               Here we retrieve ownership for the read functions
                let reinsert_config: Config<Format>;
                let defaulted: bool;

                match config {
                    Config::File(mut config_file) => {
                        config_file = config_file.read()?;
                        defaulted = config_file.defaulted;
                        reinsert_config = Config::File(config_file);
                    },
                    Config::Directory(mut config_dir) => {
                        config_dir = config_dir.read()?;
                        defaulted = config_dir.defaulted;
                        reinsert_config = Config::Directory(config_dir);
                    }
                }

                if defaulted {
                    self.defaulted = true;
                }

                self.configs.insert(key, reinsert_config);
            }
        }

        if self.defaulted && self.options.write_if_defaulted {
            self = self.write()?;
        }
        
        Ok(self)
    }

    pub fn write(mut self) -> Result<Self, Error> {
        config::ensure(&self.path)?;
    //  ^^^^^^^^^^^^^^^ Calling write on a ConfigFile already ensures the directory exists.
    //                  However, if we call write on an empty ConfigDirectory,
    //                  we still want the directory to be made

        for key in self.children() {
            // We should only write directory contents if recursive is enabled.
            // Since the write functions require that we retrieve ownership,
            // we want to be sure beforehand if we're supposed to write the config,
            // which we figure out here
            let config: &Config<Format> = self.configs.get(&key).unwrap();
            let should_write: bool = if let Config::Directory(config_dir) = config {
                if self.options.recursive {
                    true
                } else {
                    config::ensure(&config_dir.path)?;
                //  ^^^^^^^^^^^^^^^^^^^^^ If we're not going to write directory contents,
                //                        we still want the directory to be made
                    false
                }
            } else {
                true
            };

            if should_write {
                let config: Config<Format> = self.configs.remove(&key).unwrap();
            //                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            //                               Here we retrieve ownership for the write functions

                match config {
                    Config::File(config_file) => {
                        self.configs.insert(key, Config::File(config_file.write()?));
                    },
                    Config::Directory(config_dir) => {
                        self.configs.insert(key, Config::Directory(config_dir.write()?));
                    }
                }
            }
        }

        Ok(self)
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test::test_path::TestPath;
    use crate::test::child_path::ChildPath;
    use crate::config_file::ConfigFile;
    use crate::formats::string_format::StringFormat;

    #[test]
    fn new_directory() {
        ConfigDirectory::new(Path::new("test"), StringFormat::new());
    }

    #[test]
    fn insert_file() {
        ConfigDirectory::new(Path::new("test"), StringFormat::new())
            .file(ConfigFile::new(Path::new("test.txt"), StringFormat::new()));
    }

    #[test]
    fn insert_dir() {
        ConfigDirectory::new(Path::new("test"), StringFormat::new())
            .dir(ConfigDirectory::new(Path::new("test"), StringFormat::new()));
    }

    #[test]
    fn ensure_parent() {
        let tp: TestPath = TestPath::new();
        let p1: &Path = &tp.path;
        let p2: &Path = &tp.child_path("test");

        ConfigDirectory::new(p2, StringFormat::new())
            .write()
            .unwrap();
        
        assert!(p1.is_dir());
    }

}