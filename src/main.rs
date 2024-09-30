use std::{env, process}; // Consume std library modules.

use minigrep::Config; // Consume sister-library module.

fn main() {
    // RHS uses iterator and collect traits.
    // LHS uses std collection type.
    let args: Vec<String> = env::args().collect();

    // Demonstrates conditional assignment provided by unwrapping a Result enum.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Example of conditional-let, without the need for unpacking.
    // Note the Box parametric type uses the "dyn" keyword. I.e. runtime binding of the particular error type.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
