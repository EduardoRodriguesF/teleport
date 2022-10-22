use std::env;

use teleport::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    Config::build(&args);
}
