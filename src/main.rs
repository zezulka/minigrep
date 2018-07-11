extern crate minigrep;
use minigrep::Config;

fn main() {
    if let Err(e) = minigrep::run(Config::new()) {
        eprintln!("Application error : {}", e);
    }
}
