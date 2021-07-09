use std::{env, process};
use kskgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Проблема при вводе аргументов: {}", err);
        process::exit(1);
    });

    if let Err(e) = kskgrep::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}