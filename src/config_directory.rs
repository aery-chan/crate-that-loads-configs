use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use std::io::Error;
use std::fs;

use crate::format;
use crate::config_file::ConfigFile;
use crate::config::Config;

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

    format: Format,
    options: ConfigDirOpts
}

impl<Format: format::Format + Sized + Clone> ConfigDirectory<Format> {

    pub fn new(path: &Path, format: Format) -> Self {
        Self {
            path: path.to_path_buf().into_boxed_path(),
            configs: HashMap::new(),

            format,
            options: ConfigDirOpts::default()
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

    pub fn file(mut self, mut config_file: ConfigFile<Format>) -> Self {
        config_file.path = self.dir_path(&config_file.path);
        self.configs.insert(self.config_name(&config_file.path), Config::File(config_file));
        self
    }

    pub fn dir(mut self, mut config_dir: ConfigDirectory<Format>) -> Self {
        config_dir.path = self.dir_path(&config_dir.path);
        self.configs.insert(self.config_name(&config_dir.path), Config::Directory(config_dir));
        self
    }

    #[allow(clippy::ptr_arg)]
    fn has_config(&mut self, file_name: &String) -> Option<String> {
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

        found_key
    }

    /// Ensures that directory exists in fs
    fn ensure(&self) -> Result<(), Error> {
        if !self.path.is_dir() {
            fs::create_dir(&self.path)?;
        }
        Ok(())
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

    /* TODO
    if read_new and self.path.exists {
        for file in fs:read(self.path) {
            if not file exits in self.configs {
                add file to self.configs
            }
        }
    }

    let defaulted

    for config in self.configs {
        if config path is in self.path {
            if config is file
            or config is directory and recursive {
                config.read()

                if config.defaulted {
                    defaulted = true
                }
            }
        }
    }
    */
    pub fn read(mut self) -> Result<Self, Error> {
        for entry in fs::read_dir(&self.path)? {
            let entry: fs::DirEntry = entry?;
            let config_name: String = entry.file_name().into_string().unwrap();

            match self.has_config(&(&entry).file_name().into_string().unwrap()) {
                Some(key) => {
                    match self.configs.remove(&key).unwrap() {
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

    pub fn write(mut self) -> Result<Self, Error> {
        self.ensure()?;

        for key in self.children() {
            // If config is a directory and recursive isn't enabled, we shouldn't write the directory contents.
            // Here we figure out if we should write the config
            let config: &Config<Format> = self.configs.get(&key).unwrap();
            let should_write: bool = if let Config::Directory(config_dir) = config {
                if self.options.recursive {
                    true
                } else {
                    config_dir.ensure()?;
                //  ^^^^^^^^^^^^^^^^^^^^^ If we're not going to write directory contents,
                //                        we still want to make sure the directory exists
                    false
                }
            } else {
                true
            };

            // Since we're going to be retrieving ownership of the config bellow,
            // we want to be sure we're even supposed to be using it
            if should_write {
                let config: Config<Format> = self.configs.remove(&key).unwrap();
            //                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            //  Since the write functions bellow require ownership we need to retrieve ownership here

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

}