use std::fs::File;
use std::io::Write;

pub fn cmakelists_txt(project_name: &str, cxx_std: i32) {
    let mut file = File::create("CMakeLists.txt").unwrap();
    let origin_content = include_str!("../files/CMakeLists.txt");
    // Replace project_name and cxx_std in the content
    let projname_content = origin_content.replace("{{PROJECT_NAME}}", project_name);
    let content = projname_content.replace("{{cxx_std}}", &cxx_std.to_string());
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
        assert!(content.contains("project(minesweeper VERSION 0.0.0)"));
    }
}
