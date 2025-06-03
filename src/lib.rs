use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;

    let results = if config.ignore_case() {
        search_case_insensitive(&config.query(), &contents)
    } else {
        search(&config.query(), &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    /// Builds Config from provided command line arguments
    /// Note: May not behave as expected if there are exactly two arguments and the second argument contains "help". In such a case, it will return usage guidelines for the user and exit the process with code 0.
    pub fn build(args: &[String]) -> Result<Self, String> {
        let length = args.len();

        // display help for the command
        if length == 2 && args[1].to_lowercase().contains(&"help".to_lowercase()) {
            println!("Usage: minigrep [query] [file path] (ignore_case)");
            process::exit(0);
        }

        if length < 3 {
            return Err(format!(
                "Not enough arguments provided: Expected 3 found {}",
                length
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case: bool =
            length >= 4 && args[3] == "ignore_case" || env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
