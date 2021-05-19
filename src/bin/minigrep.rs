use std::env;
use std::fs;

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let filename = &args[2];

    println!("Searching for {}", pattern);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    println!("Text: {}", contents);
}
