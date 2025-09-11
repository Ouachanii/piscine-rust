use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    if let Ok(p) = std::fs::read_to_string(path) {
        println!("File content: {}", p);
    } else {
        std::fs::write(path, content).unwrap();
    }
}