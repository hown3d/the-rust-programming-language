use std::{env, process};

use cli::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
