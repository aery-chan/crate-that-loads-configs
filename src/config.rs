#[path = "./format.rs"]
mod format;

use std::path::Path;

pub struct Config<'a, Content> {
    file_path: &'a Path,
    content: Option<Content>,
    format: Box<dyn format::Format<Content>>
}

impl<'a, Content> Config<'a, Content> {

    pub fn new(file_path: &'a Path, format: Box<dyn format::Format<Content>>) -> Self {
        Self {
            file_path: file_path,
            content: None,
            format: format
        }
    }

    pub fn read(&mut self) {
        
    }

    pub fn write(&mut self) {
        
    }
    
}