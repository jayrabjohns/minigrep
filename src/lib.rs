use std::{env, error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pattern: String,
    file_path: std::path::PathBuf,

    /// Ignore case
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file_path)?;
    let ignore_case = args.ignore_case || env::var("IGNORE_CASE").is_ok();
    let results = if ignore_case {
        search_case_insensitive(&args.pattern, &contents)
    } else {
        search(&args.pattern, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect::<Vec<_>>()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    map_search(query.to_lowercase().as_str(), contents, &str::to_lowercase)
}

fn map_search<'a>(
    query: &str,
    contents: &'a str,
    pre_compare_fn: &impl Fn(&str) -> String,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| pre_compare_fn(line).contains(query))
        .collect::<Vec<_>>()
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
