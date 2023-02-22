//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


use std::{
    error::Error,
    fs::{self}, env,
};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

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

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(list: &[String]) -> Result<Config, &'static str> {
        if list.len() < 3 {
            return Err("请提供查询的关键字和文件路径");
        }

        let query = list[1].clone();
        let file_path = list[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut ret = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      ret.push(line);
    }
  }

  ret
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut ret = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      ret.push(line);
    }
  }

  ret
}


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "product";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }


    #[test]
    fn case_sensitive() {
        let query = "product";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }


    #[test]
    fn case_insensitive() {
        let query = "Productive";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(&query, &contents));
    }
}
