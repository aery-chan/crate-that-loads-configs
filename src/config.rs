use crate::format;
use crate::config_file::ConfigFile;
use crate::config_directory::ConfigDirectory;

pub enum Config<Format: format::Format + Sized + Clone> {
    File(ConfigFile<Format>),
    Directory(ConfigDirectory<Format>)
}