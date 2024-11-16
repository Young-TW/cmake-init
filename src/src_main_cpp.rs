use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn src_main_cpp() {
    let src_dir = Path::new("./src");
    let main_cpp_path = src_dir.join("main.cpp");

    if !src_dir.exists() {
        fs::create_dir_all(src_dir).expect("Failed to create src directory");
    }

    if main_cpp_path.exists() {
        return;
    }

    let content = include_str!("../files/main.cpp");
    let mut file = File::create(&main_cpp_path).expect("Failed to create main.cpp");
    file.write_all(content.as_bytes())
        .expect("Failed to write to main.cpp");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_src_main_cpp() {
        src_main_cpp();
        let content = std::fs::read_to_string("./src/main.cpp").unwrap();
        assert!(content.contains("int main(int argc, char* argv[]) {"));
    }
}
