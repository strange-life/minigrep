use std::{error::Error, fs};

pub struct Config {
    pub path: String,
    pub query: String,
}

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let mut config = Config {
        path: String::new(),
        query: String::new(),
    };

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
    let content = fs::read_to_string(&config.path)?;

    for line in search(&content, &config.query) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(content: &'a str, query: &'a str) -> impl Iterator<Item = &'a str> {
    content.lines().filter(move |line| line.contains(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let args = vec![
            String::from("-p"),
            String::from("path"),
            String::from("-q"),
            String::from("query"),
        ]
        .into_iter();

        let config = parse_config(args).unwrap();

        assert_eq!(config.path.as_str(), "path");
        assert_eq!(config.query.as_str(), "query");
    }

    #[test]
    fn one_result() {
        let content = "Rust:
safe, fast, productive.
Pick three.";
        let query = "duct";
        let results: Vec<_> = search(content, query).collect();

        assert_eq!(results, vec!["safe, fast, productive."]);
    }
}
