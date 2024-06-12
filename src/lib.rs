use std::{env, error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub query: String,
}

pub fn get_config() -> Result<Config, &'static str> {
    let mut args = env::args().skip(1);

    let Some(file_path) = args.next() else {
        return Err("no file path");
    };

    let Some(query) = args.next() else {
        return Err("no query");
    };

    Ok(Config { file_path, query })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&content, &config.query) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let content = "Rust:
safe, fast, productive.
Pick three.";

        let query = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(content, query));
    }
}
