#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod format;
pub mod formats;
pub mod config_file;
pub mod config_directory;
pub mod config;

#[cfg(test)]
pub mod test;

fn main() {}