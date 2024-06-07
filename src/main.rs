use minigrep::get_config;
use minigrep::run;
use std::process;

fn main() {
    let config = get_config().unwrap_or_else(|e| {
        eprintln!("Parse arguments failed: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Search failed: {e}");
        process::exit(1);
    }
}
