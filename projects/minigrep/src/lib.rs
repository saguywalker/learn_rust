use std::error::Error;
use std::fs;

pub struct Config{
    query: String,
    filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {query, filename}
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    
    Ok(())
}