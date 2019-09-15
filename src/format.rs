pub struct Deserialized<Content>(pub Content, pub bool);

pub trait Format {

    type Content;
    type Defaults: Clone;

    fn deserialize(&mut self, input: Vec<u8>, defaults: Option<&Self::Defaults>) -> Deserialized<Self::Content>;

    fn serialize(&mut self, input: Option<&Self::Content>) -> Vec<u8>;

}