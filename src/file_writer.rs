use std::fs::OpenOptions;
use std::io::Write;

pub fn append(path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Unable to open file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}
