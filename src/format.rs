pub trait Format<Read, Write, Defaults> {

    fn read(input: Vec<u8>, defaults: Defaults) -> Read;

    fn write(input: Write) -> Vec<u8>;

}