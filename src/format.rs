pub trait Format<Content> {

    fn serialize(&self, input: Vec<u8>, defaults: Content) -> Content;

    fn deserialize(&self, input: Content) -> Vec<u8>;

}