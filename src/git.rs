//! Initializes a Git repository in the current directory.

use std::process::Command;

/// Runs `git init` in the current directory.
///
/// Reports the outcome to standard output on success, or to standard error if
/// `git` exits with a failure status or cannot be executed (for example, when
/// `git` is not installed). Errors are not propagated to the caller.
pub fn git_init() {
    let output = Command::new("git").arg("init").output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Initialized empty Git repository.");
        }
        Ok(output) => {
            eprintln!(
                "Failed to initialize Git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute git init: {}. Is git installed?", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::in_temp_dir;

    #[test]
    fn test_git_init_creates_repository() {
        // Skip when git is unavailable so the suite stays portable.
        if Command::new("git").arg("--version").output().is_err() {
            return;
        }

        in_temp_dir(|| {
            git_init();
            assert!(std::path::Path::new(".git").exists());
        });
    }
}
