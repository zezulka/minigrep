extern crate argparse;

use std::fs::File;
use std::io::prelude::*; // I/O traits
use std::error::Error;
use std::env;

#[cfg(test)]
mod test {
    use super::*;

    // Functionality for the search feature.
    #[test]
    fn case_sensitive() {
        let query = "who";
        let contents = "\
lorem ipsum
word
as a Whole
a whopping number of rats";
        assert_eq!(vec!["a whopping number of rats"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "whO";
        let contents = "\
lorem ipsum
word
as a Whole
a whopping number of rats";
        assert_eq!(vec!["as a Whole", "a whopping number of rats"],
                   search_case_insensitive(query, &contents));
    }
}

// Here we are saying that the lifetime of the return value is connected to the lifetime of the
// contents variable
pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = vec![];
    // We cannot call the lowercase on the contents itself as the string slices would be invalid
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

pub struct Config {
    file_path : String,
    pattern   : String,
    case_sensitive : bool
}

// Struct which binds data together passed by the user.
impl Config {
    pub fn new() -> Config {
        use argparse::{ArgumentParser, Store};
        let mut args = Config { file_path : String::new(), pattern : String::new(), case_sensitive : env::var("CASE_INSENSITIVE").is_err() };
        {  // this block limits the scope for the ap.refer method
            let mut ap = ArgumentParser::new();
            ap.set_description("Yet another grep-like tool which is being programmed for teaching purposes only.");
            ap.refer(&mut args.pattern)
                .add_argument("pattern", Store, "Pattern to search inside the file for.")
                .required();
            ap.refer(&mut args.file_path)
                .add_argument("file", Store, "File to be grepped.")
                .required();
            ap.parse_args_or_exit();
        }
        args
    }
}

pub fn run(cfg : Config) -> Result<(), Box<Error>> {
    let mut buff = String::new();
    File::open(&cfg.file_path)?.read_to_string(&mut buff)?;

    let results = if cfg.case_sensitive {
        search(&cfg.pattern, &buff)
    } else {
        search_case_insensitive(&cfg.pattern, &buff)
    };

    for line in results {
         println!("{}", line);
    }

    Ok(())
}
