#[cfg(test)]
mod test {
    use super::*;

    // Functionality for the search feature.
    #[test]
    fn one_result() {
        let query = "who";
        let contents = "\
lorem ipsum
word
as a Whole
a whopping number of rats";
        assert_eq!(vec!["a whopping number of rats"], search(query, contents));
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

extern crate argparse;

pub struct Config {
    file_path : String,
    pattern   : String
}

// Struct which binds data together passed by the user.
impl Config {
    pub fn new() -> Config {
        use argparse::{ArgumentParser, Store};
        let mut args = Config { file_path : String::new(), pattern : String::new() };
        {  // this block limits the scope for the ap.refer method
            let mut ap = ArgumentParser::new();
            ap.set_description("Yet another grep-like tool which is being programmed for teaching purposes only.");
            ap.refer(&mut args.file_path)
                .add_argument("file", Store, "File to be grepped.")
                .required();
            ap.refer(&mut args.pattern)
                .add_argument("pattern", Store, "Pattern to search inside the file for.")
                .required();
            ap.parse_args_or_exit();
        }
        args
    }
}

use std::fs::File;
use std::io::prelude::*; // I/O traits
use std::error::Error;
pub fn run(cfg : Config) -> Result<(), Box<Error>> {
    let mut buff = String::new();
    File::open(&cfg.file_path)?.read_to_string(&mut buff)?;

    for line in search(&cfg.pattern, &buff) {
         println!("{}", line);
    }

    Ok(())
}
