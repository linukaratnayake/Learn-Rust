use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // args[0] is the program name.
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        let ignore_case_cli = is_cli_ignore_case_set(&args);

        let ignore_case = decide_ignore_case(ignore_case_env, ignore_case_cli);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn is_cli_ignore_case_set(args: &[String]) -> Option<bool> {
    if args.len() < 4 {
        return None;
    }

    let Some((prefix, suffix)) = args[3].split_once('=') else {
        return None;
    };

    if prefix.eq("IGNORE_CASE") && suffix.eq("TRUE") {
        return Some(true);
    } else if prefix.eq("IGNORE_CASE") && suffix.eq("FALSE") {
        return Some(false);
    }

    None
}

fn decide_ignore_case(env_set: bool, cli_set: Option<bool>) -> bool {
    match cli_set {
        Some(cli_set) => cli_set,
        None => env_set,
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}
