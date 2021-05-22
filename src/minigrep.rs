use std::error::Error;
use std::fs;

pub struct Config {
    pub pattern: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("minigrep: need exactly at least 2 arguments.");
        }

        let pattern = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { pattern, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
