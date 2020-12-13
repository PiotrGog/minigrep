use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Given args: {:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("Filename: {}", filename);
}
