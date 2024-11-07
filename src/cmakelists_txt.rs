use std::fs::File;
use std::io::Write;

pub fn cmakelists_txt(project_name: &str, cxx_std: i32) {
    let mut file = File::create("CMakeLists.txt").unwrap();
    let content = include_str!("../files/CMakeLists.txt");
    file.write_all(content.as_bytes()).unwrap();
}
