use std::fs; // allows for file system manipulation
use std::error::Error; // processes errors by either return a Result with its type or an Error 
use std::env;


impl Config {
    // returns a Config instance in the success case and a &'static str in the error case
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

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, // 
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

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

// 'a paramter specifies which argument is connected to the the lifetime of the return value
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // stores ref data from for loop and places into vector array
    let mut results = Vec::new();
    // for loop iterates over each word in contents and if line == query,
    // line is pushed to results vector array
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
// searches contents for query but lowercases query string so if equality is met line is pushed to resutls vec
pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_ascii_lowercase().contains(&query) {
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
        let contents = "/
    
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

