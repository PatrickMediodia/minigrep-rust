use std::env;
use std::process;

use minigrep_rust;

fn main() {
    // get the arguments passsed through the command line
    let args: Vec<String> = env::args().collect();

    // Prints and returns the value of a given expression for quick and dirty debugging.
    // dbg!(args);

    // use the build method constructor on the config struct
    // unwrap_or_else allows us to define non-panic error handling
    // if successful, gives the Ok(config) value
    // if not, gives the Err(message) value inside the closure
    let config = minigrep_rust::Config::build(&args).unwrap_or_else(|err| {
        // prints to the standard outout
        //println!("Problem parsing arguments: {err}");
        
        // prints to the standard error
        eprintln!("Problem parsing arguments: {err}");


        // nonzero exit status to signal that our program excited with an error state
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // since our run function does not return a value if it is successful
    // just handle the error case
    if let Err(e) = minigrep_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/* 
    Test-Driven Development Steps
    1. Write a test that fails and run it to make sure it fails for the reason you expect.
    2. Write or modify just enough code to make the new test pass.
    3. Refactor the code you just added or changed and make sure the tests continue to pass.
    4. Repeat from step 1!

*/

// sets ignore_case to to 1
// IGNORE_CASE=1 cargo run -- to poem.txt

// print to standard error and not standard output
// eprintln!("Problem parsing arguments: {err}");

// pipe the output of the library to an output file
// cargo run -- to poem.txt > output.txt
