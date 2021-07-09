use std::fs;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Не хватает аргуметов! kskgrep искомое где_искать");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("KSKGREP_CASE_INSENSITIVE").is_err()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "просто";
        let content = "\
Язык програмирования Rust!
это самый быстрый, надёжный и удобный
язык, за ним просто и легко программировать.
Сейчас я пишу тест для функции search";

        assert_eq!(vec!["язык, за ним просто и легко программировать."], search(query, content));
    }

    #[test]
    fn case_insensitive(){
        let query = "язык";
        let content = "\
Язык програмирования Rust!
это самый быстрый, надёжный и удобный
язык, за ним просто и легко программировать.
Сейчас я пишу тест для функции search";

        assert_eq!(vec!["Язык програмирования Rust!", "язык, за ним просто и легко программировать."], search_case_insensitive(query, content));
    }
}
