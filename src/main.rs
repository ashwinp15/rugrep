use std::{env, process};
use rugrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = rugrep::run(&config) {
        println!("Application error: {error}");
    }

}
