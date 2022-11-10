use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrip::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrip::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
