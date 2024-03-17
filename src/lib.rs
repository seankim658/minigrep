use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for (i, line) in search(&config.query, &contents, config.ignore_case).iter().enumerate() {
        println!("Match #{} on line {}: {}", i + 1, line.0 + 1, line.1);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    for (i, line) in contents.lines().enumerate() {
        let line_to_check = match ignore_case {
            true => line.to_lowercase(),
            false => line.to_string(),
        };
        if line_to_check.contains(&query) {
            results.push((i, line));
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
        assert_eq!(vec![(1, "safe, fast, productive.")], search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec![(0, "Rust:")], search(query, contents, true));
    }
}
