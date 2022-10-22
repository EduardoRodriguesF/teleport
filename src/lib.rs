pub struct Config {
    pub key: str,
}

impl Config {
    pub fn build(args: &[String]) {
        println!("{:?}", args);
    }
}
