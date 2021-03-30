use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> =env::args().collect();

    let config = lib::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); // exits process without panic
    });
    println!("Searching for {}", config.query);
    println!("Searching in: {}", config.filename);
    if let Err(err) = lib::run(config) {
        println!("Error: {}", err);
        process::exit(1);
    };

}




