use std::path::PathBuf;

const SITES_ROOT: &'static str = "LIGHT_LETTER_SITES_ROOT";

mod sites_config;

pub fn start(sites_root: PathBuf) {
    let config = sites_config::read_sites_config(&sites_root);
    dbg!(config);
}

#[allow(dead_code)]
fn main() {
    let sites_root = PathBuf::from(std::env::var(SITES_ROOT).unwrap_or_else(|_| String::from(".")));
    start(sites_root);
}
