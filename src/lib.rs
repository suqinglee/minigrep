use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in if config.sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    } {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub sensitive: bool,
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let sensitive = env::var("INSENSITIVE").is_err();

        Ok(Config { sensitive, query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(&query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}