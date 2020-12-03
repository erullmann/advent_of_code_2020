use std::env;
use std::process;

use advent_of_code_2020::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arugments: {}", err);
        process::exit(1);
    });
    
    advent_of_code_2020::run(config)
}
