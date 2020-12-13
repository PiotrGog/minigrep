use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Given args: {:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Given file content:\n{}", content);
}
