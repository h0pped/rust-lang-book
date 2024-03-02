use std::env;
use std::process;

use minigrep_cli::Config;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    eprintln!("Searching for: {}", config.query);
    eprintln!("In file: {}\n", config.filename);

    if let Err(e) = minigrep_cli::run(config) {
        eprintln!("\nApplication error: {}", e);
        process::exit(1);
    }
}
