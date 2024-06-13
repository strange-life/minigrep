use minigrep::{parse_config, run};
use std::{env, process};

fn main() {
    let config = parse_config(env::args()).unwrap_or_else(|e| {
        eprintln!("Parse arguments failed: {e}");
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Search failed: {e}");
        process::exit(1);
    }
}
