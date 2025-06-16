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
        _ => {
            origin_content = include_str!("../files/CMakeLists.txt");

        }
    }

    // Check if the file already exists
    if std::path::Path::new("CMakeLists.txt").exists() {
        eprintln!("CMakeLists.txt already exists. Skipping creation.");
        return;
    }
    // Replace project_name and cxx_std in the content
    let projname_content = origin_content.replace("{{PROJECT_NAME}}", project_name);
    let content = projname_content.replace("{{cxx_std}}", &cxx_std.to_string());
    file.write_all(content.as_bytes()).unwrap();
}
