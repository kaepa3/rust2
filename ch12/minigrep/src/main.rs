extern crate minigrep;

use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing:{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("App error:{}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            minigrep::search(query, contents)
        );
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            minigrep::search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            minigrep::search_insensitive(query, contents)
        );
    }
}
>>>>>>> 16df5038968d8a233a6e7b753fc779f605c64ec7
