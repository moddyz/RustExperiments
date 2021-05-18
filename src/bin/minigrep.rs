use std::env;

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let filename = &args[2];

    println!("Searching for {}", pattern);
    println!("In file {}", filename);
}
