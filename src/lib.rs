use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Note return type is a generic Result<>.
    // Note the way the type parameter syntax for "reference to a string const"
    //
    // See also the Functional Language chapter in the book - which shows how
    // ownership complexity here could be avoided, if an iterator was passed in,
    // instead of passing in a slice. That would avoid the need for this function
    // to take ownership of the slice.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone() chosen as a trade off. The performance hit merits the ownership convenience.
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return an Ok() - part of the Result enum system.
        Ok(Config { query, file_path })
    }
}

// Note exporting (pub) this fn.
// Note the Boxing type params:
// First is unit object.
// Second is a "dynamic" error - i.e. runtime binding type.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

// Note the use of lifetime specifiers. The function parameters are references and
// so is the returned value. The lifetime controls tells the compiler, how long
// the heap data these point too must be preserved.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Note how the type parameter for this Vec is not specified, but can be
    // implied from code that comes LATER.
    let mut results = Vec::new();
    // Note the line scanning method available on &str - returns an Iterator.
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

// This is not used - but shows a more concise and idiomatic way to do it.
//
// Note the functional programming style "filter" and "collect" traits being available
// on an Iterator.
pub fn search_alternative<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
// This is declaring a local sub-module called test and defining said module inline.
mod tests {
    use super::*; // Bring the parent scope into this submodule (tests) module.

    // This declares the function to be a test (explicit, not a naming convention)
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // todo - unclear what equal means for references to Vectors?
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
