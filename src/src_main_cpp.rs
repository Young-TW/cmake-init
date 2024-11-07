use std::fs::File;
use std::io::Write;

pub fn src_main_cpp() {
    let mut file = File::create("./src/main.cpp").unwrap();
    let content = include_str!("../files/main.cpp");
    file.write_all(content.as_bytes()).unwrap();
}
