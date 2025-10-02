use std::fs;
use std::error::Error;

// Return a Result<T, E> when something goes wrong
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Many developers expect 'new' functions to never fail, so moved to 'build' instead
    // We either return a Config, or an error string with lifetime 'static
    // Our error values will always be string literals with the 'static lifetime.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // improve 'index out of bounds' error to something more meaningful
        if args.len() < 3  { // 3 because script path, arg1, arg2
            return Err("Not enough arguments, 2 required.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Data referenced by a slice needs to be valid, using lifetimes to ensure this 
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();
 
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    } 

    // return the results
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // first set everything to lowercase so we can compare apples with apples
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    } 

    // return the results
    results
}

// 2025-10-01 Add a Failing test (Starting with TDD)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
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
