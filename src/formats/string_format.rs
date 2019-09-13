#[path = "../format.rs"]
mod format;

use format::Format;

pub struct StringFormat;

impl Format<String> for StringFormat {

    fn deserialize(&mut self, input: Vec<u8>, defaults: &Option<String>) -> String {
        if input.len() > 0 {
            match String::from_utf8(input) {
                Ok(__input) => __input,
                Err(err) => panic!(err)
            }
        } else {
            match defaults.clone() {
                Some(__defaults) => __defaults,
                None => String::new()
            }
        }
    }

    fn serialize(&mut self, input: &Option<String>) -> Vec<u8> {
        match input {
            Some(__input) => __input.as_bytes().to_vec(),
            None => Vec::new()
        }
    }

}

impl StringFormat {
    fn new() -> Self {
        Self {}
    }
}