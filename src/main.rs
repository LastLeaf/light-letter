use std::path::PathBuf;
use light_letter;

const SITES_ROOT: &'static str = "LIGHT_LETTER_SITES_ROOT";

fn main() {
    env_logger::init();
    let sites_root = PathBuf::from(std::env::var(SITES_ROOT).unwrap_or_else(|_| String::from(".")));
    light_letter::start(sites_root);
}
