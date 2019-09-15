use crate::format::{ Format, Deserialized };

pub struct StringFormat;

#[allow(clippy::new_without_default)]
impl StringFormat {
    pub fn new() -> Self {
        Self {}
    }
}

impl Format for StringFormat {

    type Content = String;
    type Defaults = String;

    fn deserialize(&mut self, input: Vec<u8>, defaults: Option<&Self::Defaults>) -> Deserialized<Self::Content> {
        if !input.is_empty() {
            match String::from_utf8(input) {
                Ok(__input) => Deserialized(__input, false),
                Err(err) => panic!(err)
            }
        } else {
            Deserialized(match defaults {
                Some(__defaults) => __defaults.clone(),
                None => String::new()
            }, true)
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
        assert_eq!(f.deserialize(s.as_bytes().to_vec(), None).0, s);
    }

    #[test]
    fn deserialize_defaults() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.deserialize(vec![], Some(&s)).0, s);
    }

    #[test]
    fn serialize_string_to_bytes() {
        let mut f: StringFormat = StringFormat::new();
        let s: String = String::from("Hello, world!");
        assert_eq!(f.serialize(Some(&s)), s.as_bytes().to_vec());
    }

}