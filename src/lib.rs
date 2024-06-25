use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Config {
    pub path: String,
    pub query: String,
    pub ignore_case: bool,
}

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let mut config = Config {
        path: String::new(),
        query: String::new(),
        ignore_case: false,
    };

    config.ignore_case = env::var("IGNORE_CASE").is_ok();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" | "--path" => {
                if let Some(path) = args.next() {
                    config.path = path;
                }
            }
            "-q" | "--query" => {
                if let Some(query) = args.next() {
                    config.query = query;
                }
            }
            "-i" | "--ignore_case" => {
                config.ignore_case = true;
            }
            _ => (),
        }
    }

    if config.path.is_empty() {
        return Err("path required");
    }

    if config.query.is_empty() {
        return Err("query required");
    }

    Ok(config)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let lines = BufReader::new(File::open(&config.path)?).lines();

    for line in search(lines, &config.query, config.ignore_case) {
        println!("{line}", line = line?);
    }

    Ok(())
}

fn search<'a, T>(
    lines: T,
    query: &'a str,
    ignore_case: bool,
) -> impl Iterator<Item = Result<String, io::Error>> + 'a
where
    T: Iterator<Item = Result<String, io::Error>> + 'a,
{
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    lines.filter(move |line| match line {
        Ok(line) => {
            if ignore_case {
                line.to_lowercase().contains(&query)
            } else {
                line.contains(&query)
            }
        }
        Err(_) => true,
    })
}
