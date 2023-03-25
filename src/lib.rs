use std::fs; // allows for file system manipulation
use std::error::Error; // processes errors by either return a Result with its type or an Error 


pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // returns a Config instance in the success case and a &'static str in the error case
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
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
}

