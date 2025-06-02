use std::fs;

pub fn run(config: Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let length = args.len();

        if length < 3 {
            return Err(format!(
                "Not enough arguments provided: Expected 3 found {}",
                length
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn file_path(&self) -> String {
        self.file_path.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
