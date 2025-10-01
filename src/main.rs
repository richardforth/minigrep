use std::env;
use std::fs;
use std::process;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("E: Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("E: {e}");
        process::exit(1);
    }
}

// Return a Result<T, E> when something goes wrong
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}


struct Config {
    query: String,
    file_path: String,
}


impl Config {
    // Many developers expect 'new' functions to never fail, so moved to 'build' instead
    // We either return a Config, or an error string with lifetime 'static
    // Our error values will always be string literals with the 'static lifetime.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // improve 'index out of bounds' error to something more meaningful
        if args.len() < 3  { // 3 because script path, arg1, arg2
            return Err("Not enough arguments, 2 required.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
