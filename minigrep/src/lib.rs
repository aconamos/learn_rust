use std::{
    env,
    error::Error,
    fs,
    io::{Read, Stdin},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Config {
    query: String,
    input: InputStream,
    ignore_case: bool,
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let query = args.nth(1).ok_or("Couldn't extract thing!")?;

        let input = match args.next() {
            Some(str) => match str.as_str() {
                "-" => InputStream::StdInStream(std::io::stdin()),
                _ => InputStream::FilePath(str),
            },
            _ => InputStream::StdInStream(std::io::stdin()),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            input,
            ignore_case,
        })
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
    contents.lines().filter(|s| s.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|s| s.to_lowercase().contains(&query))
        .collect()
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
