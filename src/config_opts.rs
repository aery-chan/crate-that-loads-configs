pub struct ConfigOpts {
    write_if_defaulted: bool
}

impl Default for ConfigOpts {

    fn default() -> Self {
        Self {
            write_if_defaulted: false
        }
    }
    
}