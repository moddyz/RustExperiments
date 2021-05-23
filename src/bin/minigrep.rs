use std::env;
use std::process;

extern crate rustsandbox;

use rustsandbox::minigrep::Config;

fn main() {
    // Collect command line arguments.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.pattern);
    println!("In file {}", config.filename);

    if let Err(e) = rustsandbox::minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
