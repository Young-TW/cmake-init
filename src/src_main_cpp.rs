use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn src_main_cpp(mode: Option<&str>) {
    let src_dir = Path::new("./src");
    let (main_path, content);

    match mode {
        Some("CUDA") => {
            main_path = src_dir.join("main.cu");
            content = include_str!("../files/cuda/main.cu");
            if !src_dir.exists() {
                fs::create_dir_all(src_dir).expect("Failed to create src directory");
            }

            if main_path.exists() {
                return;
            }
        }
        Some("HIP") => {
            main_path = src_dir.join("main.hip");
            content = include_str!("../files/hip/main.hip");
            if !src_dir.exists() {
                fs::create_dir_all(src_dir).expect("Failed to create src directory");
            }

            if main_path.exists() {
                return;
            }
        }
        _ => {
            main_path = src_dir.join("main.cpp");
            content = include_str!("../files/main.cpp");
            if !src_dir.exists() {
                fs::create_dir_all(src_dir).expect("Failed to create src directory");
            }

            if main_path.exists() {
                return;
            }
        }
    }

    let mut file = File::create(&main_path).expect("Failed to create main.cpp/.cu/.hip file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to main.cpp");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_src_main_cpp() {
        src_main_cpp(Some("C++"));
        let content = std::fs::read_to_string("./src/main.cpp").unwrap();
        assert!(content.contains("int main(int argc, char* argv[]) {"));
    }

    #[test]
    fn test_src_main_cuda() {
        src_main_cpp(Some("CUDA"));
        let content = std::fs::read_to_string("./src/main.cu").unwrap();
        assert!(content.contains("__global__"));
    }

    #[test]
    fn test_src_main_hip() {
        src_main_cpp(Some("HIP"));
        let content = std::fs::read_to_string("./src/main.hip").unwrap();
        assert!(content.contains("__global__"));
    }
}
