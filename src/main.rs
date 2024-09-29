use std::{env, process}; // Consume std library packages.

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Example of conditional let.
    // Note the Box parametric type uses the "dyn" keyword. I.e. runtime binding.
    if let Err(e) = minigrep::run(config) {
        // don't need unwrap() because the Ok branch of run() returns nothing we need.
        println!("Application error: {e}");
        process::exit(1);
    }
}
