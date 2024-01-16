use std::fs;
use std::error::Error;

// Box<dyn Error> allows us to return an error without specifying the error type
// dyn is short for dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // the ? operators returns an error value for the calling function to handle
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // calling run for its side effects only and not for its return value
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {                         // success   fail
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // return an error if less than 3 arguments
            // allows function caller to have a clearer understanding of the problem
            return Err("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        // return config object if arguments passed
        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // backslash after opening double quotes tells rust not to put a newline character at the beginning of contnents
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// by using lifetimes, we tell rust that the data returned by the search function will live
// as long as the data passed into the search function in the contents argument

// we must explicitly say that the contents reference must be valid for the duration of the use of the returned vector
// connecting the arguments to the return values
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // compares the vector returned from this function to the vector in the test
    // using assert_eq! marcos

    /*
    Iterate through each line of the contents.
    Check whether the line contains our query string.
    If it does, add it to the list of values we’re returning.
    If it doesn’t, do nothing.
    Return the list of results that match.
    */

    let mut results = Vec::new();

    // lines method returns an iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/*
// since this function is used to create a new instance of the Config struct
// just create a constructor for more clear bindings
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    Config { query, file_path }
}
*/