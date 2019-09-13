pub trait Format {

    type Content;

    fn deserialize(&mut self, input: Vec<u8>, defaults: Option<&Self::Content>) -> Self::Content;

    fn serialize(&mut self, input: Option<&Self::Content>) -> Vec<u8>;

}