use std::{env, error::Error, fs};

pub struct Config {
    pub path: String,
    pub query: String,
}

pub fn get_config() -> Result<Config, &'static str> {
    let mut args = env::args().skip(1);

    let path = args.next().ok_or("no path")?;
    let query = args.next().ok_or("no query")?;

    Ok(Config { path, query })
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
    fn one_result() {
        let content = "Rust:
safe, fast, productive.
Pick three.";
        let query = "duct";
        let results: Vec<_> = search(content, query).collect();

        assert_eq!(results, vec!["safe, fast, productive."]);
    }
}
