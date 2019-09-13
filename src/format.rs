pub trait Format<Content> {

    fn serialize(&mut self, input: Vec<u8>, defaults: &Option<Content>) -> Content;

    fn deserialize(&mut self, input: &Option<Content>) -> Vec<u8>;

}