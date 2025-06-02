use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("\x1b[1mProblem parsing arguments: {err}\x1b[0m");
        process::exit(1);
    });

    // println!("Searching for {}", config.query());
    // println!("In file {}", config.file_path());

    if let Err(e) = run(config) {
        eprintln!("Failed to read file contents: {e}");
        process::exit(1);
    };
}
