use std::fs::File;
use std::io::Write;

pub fn src_main_cpp() {
    let mut file = File::create("./src/main.cpp").unwrap();
    let content = include_str!("../files/main.cpp");
    file.write_all(content.as_bytes()).unwrap();
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
