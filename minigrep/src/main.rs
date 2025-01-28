use std::{env, process};

use minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args_slice(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
