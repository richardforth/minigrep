use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("E: Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("E: {e}");
        process::exit(1);
    }
}

