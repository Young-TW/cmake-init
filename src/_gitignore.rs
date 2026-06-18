//! Writes the project's default `.gitignore`.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes the bundled default `.gitignore` into the current directory.
///
/// Does nothing if a `.gitignore` already exists, so an existing file is never
/// overwritten. The written contents are embedded from `files/.gitignore` at
/// compile time.
///
/// # Panics
///
/// Panics if the file cannot be created or written (for example, on a
/// permission error).
pub fn gitignore() {
    let path = Path::new(".gitignore");
    if path.exists() {
        return;
    }
    let mut file = File::create(path).unwrap();
    let content = include_str!("../files/.gitignore");
    file.write_all(content.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::in_temp_dir;

    #[test]
    fn writes_gitignore_when_absent() {
        in_temp_dir(|| {
            gitignore();
            let content = std::fs::read_to_string(".gitignore").unwrap();
            assert!(content.contains("build"));
        });
    }

    #[test]
    fn does_not_overwrite_existing_gitignore() {
        in_temp_dir(|| {
            std::fs::write(".gitignore", "custom content").unwrap();
            gitignore();
            let content = std::fs::read_to_string(".gitignore").unwrap();
            assert_eq!(content, "custom content");
        });
    }
}
