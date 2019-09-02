use light_letter;

fn main() {
    let mut p = std::path::PathBuf::from(file!());
    p.pop();
    light_letter::start(p);
}
