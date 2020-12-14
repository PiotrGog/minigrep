use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("Given file content:\n{}", content);
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}
