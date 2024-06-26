use lastfm_charts::config::Config;
use std::process;

fn main() {
    let config = Config::build();

    if let Err(e) = lastfm_charts::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
