use std::env;
use std::fs;

fn main() {
    // get the arguments passsed through the command line
    let args: Vec<String> = env::args().collect();

    // Prints and returns the value of a given expression for quick and dirty debugging.
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}