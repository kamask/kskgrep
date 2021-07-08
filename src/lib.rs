use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Не хватает аргуметов! kskgrep искомое где_искать");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("{}", content);
    Ok(())
}
