![Banner](https://raw.githubusercontent.com/aery-chan/crate-that-loads-configs/master/img/banner.png)
Character: Tohru from Miss Kobayashi's Dragon Maid *(Kobayashi-san Chi no Maid Dragon)*

### Crate is still in development and is not yet meant to be used

A lil' crate that simplifies your config loading needs

# Features

* **Simple**, just specify format and parameters, the crate does the rest
* **Powerful**, read or write both files and directories with defaults and numerous options
* **Flexible**, create support for formats of your needs

# Examples

### Reading a Text File
```rust
let config = Config::new(Path::new("key.txt"), StringFormat::new())
    .def("xxxx-xxxx-xxxx-xxxx") // Default content to example key
                                // if when reading, file is empty or doesnt't exist
    .opt(ConfigOpts {
        write_if_defaulted: true // Write file if content is ever defaulted
    })
    .read()
    .unwrap();

if config.defaulted {
    println!("Please enter your key in key.txt");
} else {
    println!("Your key is: {}", config.content);
}
```