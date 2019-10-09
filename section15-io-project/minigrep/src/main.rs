extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing {}", err);
        process::exit(1);
    });
    println!("{:?}", args);
    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);
    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error {}", err);
        process::exit(1);
    });;
}
