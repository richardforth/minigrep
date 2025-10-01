use std::fs;
use std::error::Error;

// Return a Result<T, E> when something goes wrong
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
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
