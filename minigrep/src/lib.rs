use std::{
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
            Ok(Config::from(
                &args[1],
                InputStream::FilePath(args[2].clone()),
            ))
        }
    }

    fn from(query: &String, input: InputStream) -> Config {
        Config {
            query: query.clone(),
            input,
        }
    }

    pub fn from_str(query: &String, input: &String) -> Config {
        Config {
            query: query.clone(),
            input: InputStream::FilePath(input.clone()),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = config.input.to_string()?;

    for line in search(&config.query, &contents) {
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
}
