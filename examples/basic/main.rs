use light_letter;

fn main() {
    env_logger::init();
    let mut p = std::path::PathBuf::from(file!());
    p.pop();
    light_letter::start(p);
}
