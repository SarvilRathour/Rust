use std::env;
use std::error::Error;
use std::fs;
use std::process;
use sarvilgrep::search;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("process exit");
        process::exit(1)
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run(config);
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query,&contents){
        println!("{line}");
    }
    println!("With text:\n{contents}");
    Ok(())
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
