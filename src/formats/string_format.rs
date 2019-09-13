use crate::format::Format;

pub struct StringFormat;

impl StringFormat {
    pub fn new() -> Self {
        Self {}
    }
}

impl Format for StringFormat {

    type Content = String;

    fn deserialize(&mut self, input: Vec<u8>, defaults: Option<&Self::Content>) -> Self::Content {
        if input.len() > 0 {
            match String::from_utf8(input) {
                Ok(__input) => __input,
                Err(err) => panic!(err)
            }
        } else {
            match defaults {
                Some(__defaults) => __defaults.clone(),
                None => String::new()
            }
        }
    }

    fn serialize(&mut self, input: Option<&Self::Content>) -> Vec<u8> {
        match input {
            Some(__input) => __input.as_bytes().to_vec(),
            None => Vec::new()
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn deserialize_bytes_to_string() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.deserialize(s.as_bytes().to_vec(), None), s);
    }

    #[test]
    fn defaults() {
        let mut f: StringFormat = StringFormat::new();
        let d: String = String::from("Hello, world!");
        assert_eq!(f.deserialize(vec![], Some(&d)), d);
    }

    #[test]
    fn serialize_string_to_bytes() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.serialize(Some(&s)), s.as_bytes().to_vec());
    }

}