use std::env;
use std::fs;

struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Config { pattern, filename }
    }
}

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.pattern);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file");
    println!("Text: {}", contents);
}
