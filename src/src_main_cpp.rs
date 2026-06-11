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
        Some("MPI") => {
            main_path = src_dir.join("main.cpp");
            content = include_str!("../files/mpi/main.cpp");
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
    use crate::test_util::in_temp_dir;

    #[test]
    fn test_src_main_cpp() {
        in_temp_dir(|| {
            src_main_cpp(Some("C++"));
            let content = std::fs::read_to_string("./src/main.cpp").unwrap();
            assert!(content.contains("int main(int argc, char* argv[]) {"));
        });
    }

    #[test]
    fn test_src_main_cuda() {
        in_temp_dir(|| {
            src_main_cpp(Some("CUDA"));
            let content = std::fs::read_to_string("./src/main.cu").unwrap();
            assert!(content.contains("__global__"));
        });
    }

    #[test]
    fn test_src_main_hip() {
        in_temp_dir(|| {
            src_main_cpp(Some("HIP"));
            let content = std::fs::read_to_string("./src/main.hip").unwrap();
            assert!(content.contains("__global__"));
        });
    }

    #[test]
    fn test_src_main_mpi() {
        in_temp_dir(|| {
            src_main_cpp(Some("MPI"));
            let content = std::fs::read_to_string("./src/main.cpp").unwrap();
            assert!(content.contains("MPI_Init"));
        });
    }

    #[test]
    fn test_src_main_existing_file_is_preserved() {
        in_temp_dir(|| {
            // First call writes the template; a second call must not overwrite an
            // already-present main file.
            src_main_cpp(Some("C++"));
            std::fs::write("./src/main.cpp", "custom content").unwrap();
            src_main_cpp(Some("C++"));
            let content = std::fs::read_to_string("./src/main.cpp").unwrap();
            assert_eq!(content, "custom content");
        });
    }
}
