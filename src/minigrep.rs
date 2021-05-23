use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Consume executable name.
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a pattern string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            pattern,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}
