use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_contents = fs::read_to_string(&config.file_path)?;
    let result = match config.ignore_case{
        true => search_case_insensitive(&config.query, &file_contents),
        false => search(&config.query, &file_contents),
    };
    for line in result{
        println!("{}", line)
    }
    Ok(())
}
fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn  case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["save, fast, productive."], search(query, contents))
    }    
    
    #[test]
    fn  case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["save, fast, productive.", "Duct tape."], search_case_insensitive(query, contents))
    }
}