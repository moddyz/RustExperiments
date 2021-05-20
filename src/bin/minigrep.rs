use std::env;
use std::process;

extern crate rustsandbox;

use rustsandbox::config::Config;

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.pattern);
    println!("In file {}", config.filename);

    if let Err(e) = rustsandbox::config::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
