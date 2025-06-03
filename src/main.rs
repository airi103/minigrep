use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("\x1b[1m\x1b[31merror:\x1b[39m {err}\x1b[0m\nUsage: minigrep [query] [file path] (ignore_case)");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("\x1b[1m\x1b[31merror:\x1b[39m {err}\x1b[0m");
        process::exit(1);
    };
}
