use std::{env, fs, process};
use std::error::Error;

pub fn read_command_line_arg() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::New(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

   if let Err(e) = run(config) {
       eprintln!("Application error: {}", e);
       process::exit(1);
   }

}

pub fn read_command_line_arg_two() {
    let config = Config::New_two(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

struct Config {
    query: String,
    fileName: String,
    case_insensitive: bool,
}

impl Config {
    fn New(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let fileName = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query, fileName, case_insensitive
        })
    }

    fn New_two(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query String"),
        };
        let fileName = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query, fileName, case_insensitive
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.fileName)?;
    println!("content value is {}", content);
    let result = if config.case_insensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in result {
        println!("line : {}", line);
    }
    Ok(())
}

pub fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_two<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}