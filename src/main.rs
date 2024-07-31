use std::env;
use std::fs::read_to_string;
use std::process::exit;

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl Config<'_> {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        let query = match args.get(1) {
            None => return Err("Search string not specified"),
            Some(s) => s,
        };
        let file_path = match args.get(2) {
            None => return Err("File path not specified"),
            Some(s) => s,
        };

        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();
    
    let contents = read_to_string(config.file_path).unwrap_or_else(|err| { println!("{}", err); exit(1) });

    println!("{contents}");

}