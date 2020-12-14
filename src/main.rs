use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Given args: {:?}", args);

    let config = Config::new(&args);

    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("Given file content:\n{}", content);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enought arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Config { query, filename };
    }
}
