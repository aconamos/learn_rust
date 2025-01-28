use std::{
    env,
    error::Error,
    fs,
    io::{Read, Stdin},
};

enum InputStream {
    FilePath(String),
    StdInStream(Stdin), // TODO: Refactor to use anything implementing the Read trait
}

impl InputStream {
    fn to_string(self) -> Result<String, Box<dyn Error>> {
        match self {
            InputStream::FilePath(str) => Ok(fs::read_to_string(str)?),
            InputStream::StdInStream(mut str) => {
                let mut contents = String::new();
                str.read_to_string(&mut contents)?;
                Ok(contents)
            }
        }
    }
}

pub struct Config {
    query: String,
    input: InputStream,
    ignore_case: bool,
}

impl Config {
    pub fn from_args_slice(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            Err("Too few arguments!")
        } else if args.len() < 3 {
            Ok(Config::from(
                &args[1],
                InputStream::StdInStream(std::io::stdin()),
            ))
        } else {
            // Usually, you would be able to pass in a '-' to indicate to use stdin anyways.
            // This program doesn't implement that because I'm lazy.
            Ok(Config::from(
                &args[1],
                InputStream::FilePath(args[2].clone()),
            ))
        }
    }

    fn from(query: &String, input: InputStream) -> Config {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query: query.clone(),
            input,
            ignore_case,
        }
    }

    pub fn from_str(query: &String, input: &String) -> Config {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query: query.clone(),
            input: InputStream::FilePath(input.clone()),
            ignore_case,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = config.input.to_string()?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
