use std::fs::File;
use std::io::Write;

pub fn gitignore() {
    let mut file = File::create(".gitignore").unwrap();
    let content = include_str!("../files/.gitignore");
    file.write_all(content.as_bytes()).unwrap();
}