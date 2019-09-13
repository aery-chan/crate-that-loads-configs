pub trait Format<Content> {

    fn deserialize(&mut self, input: Vec<u8>, defaults: &Option<Content>) -> Content;

    fn serialize(&mut self, input: &Option<Content>) -> Vec<u8>;

}