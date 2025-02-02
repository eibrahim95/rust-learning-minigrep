use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("{}", err);
        process::exit(1);
    });
    minigrep::run(config).unwrap_or_else(|err|{
        eprintln!("{}", err);
        process::exit(1);
    });
}
