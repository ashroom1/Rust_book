use command_line_project::{Config, run};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    let contents = run(config).unwrap();
    
    println!("{contents}");

}