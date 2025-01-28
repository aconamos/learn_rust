use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn from_args_slice(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Too few arguments!")
        } else {
            Ok(Config::from(&args[1], &args[2]))
        }
    }

    fn from(query: &String, file_path: &String) -> Config {
        Config {
            query: query.clone(),
            file_path: file_path.clone(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args_slice(&args).unwrap();

    println!(
        "Searching for: {}
    In file path: {}",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path).expect("File read err");

    println!("{}", contents);
}
