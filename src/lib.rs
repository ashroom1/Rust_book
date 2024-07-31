
use std::error::Error;
use std::fs::read_to_string;


pub struct Config<'a> {
	pub query: &'a str,
	pub file_path: &'a str,
}

impl Config<'_> {
	pub fn new(args: &Vec<String>) -> Result<Config, &str> {
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

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
	let contents = read_to_string(config.file_path)?;

	Ok(contents)
}
