use std::fs::File;
use std::io::Write;

pub fn cmakelists_txt(project_name: &str, cxx_std: i32, mode: Option<&str>) {
    let mut file = File::create("CMakeLists.txt").unwrap();
    let origin_content;
    match mode {
        Some("CUDA") => {
            origin_content = include_str!("../files/cuda/CMakeLists.txt");
        }
        Some("HIP") => {
            origin_content = include_str!("../files/hip/CMakeLists.txt");
        }
        Some("MPI") => {
            origin_content = include_str!("../files/mpi/CMakeLists.txt");
        }
        _ => {
            origin_content = include_str!("../files/CMakeLists.txt");
        }
    }

    // Replace project_name and cxx_std in the content
    let projname_content = origin_content.replace("{{PROJECT_NAME}}", project_name);
    let content = projname_content.replace("{{cxx_std}}", &cxx_std.to_string());
    file.write_all(content.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::in_temp_dir;

    #[test]
    fn test_cmakelists_substitutes_placeholders() {
        in_temp_dir(|| {
            cmakelists_txt("my_project", 20, Some("C++"));
            let content = std::fs::read_to_string("CMakeLists.txt").unwrap();
            assert!(content.contains("project(my_project"));
            assert!(content.contains("set(CMAKE_CXX_STANDARD 20)"));
            // Placeholders must be fully replaced.
            assert!(!content.contains("{{PROJECT_NAME}}"));
            assert!(!content.contains("{{cxx_std}}"));
        });
    }

    #[test]
    fn test_cmakelists_cuda_template() {
        in_temp_dir(|| {
            cmakelists_txt("cuda_proj", 17, Some("CUDA"));
            let content = std::fs::read_to_string("CMakeLists.txt").unwrap();
            assert!(content.contains("cuda_proj"));
            assert!(!content.contains("{{cxx_std}}"));
        });
    }
}
