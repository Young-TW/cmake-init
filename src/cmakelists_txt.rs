use std::fs::File;
use std::io::Write;

pub fn cmakelists_txt(project_name: &str, cxx_std: i32) {
    let mut file = File::create("CMakeLists.txt").unwrap();
    let content = include_str!("../files/CMakeLists.txt");
    // Replace project_name and cxx_std in the content
    let content = content.replace("{{project_name}}", project_name);
    let content = content.replace("{{cxx_std}}", &cxx_std.to_string());
    file.write_all(content.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmakelists_txt_project_name() {
        let project_name = "minesweeper";
        cmakelists_txt(project_name, 17);
        let content = std::fs::read_to_string("CMakeLists.txt").unwrap();
        assert!(content.contains("project(minesweeper)"));
    }

    #[test]
    fn test_cmakelists_txt_cxx_std() {
        let project_name = "minesweeper";
        let cxx_std = 17;
        cmakelists_txt(project_name, cxx_std);
        let content = std::fs::read_to_string("CMakeLists.txt").unwrap();
        assert!(content.contains("set(CMAKE_CXX_STANDARD 17)"));
    }
}
