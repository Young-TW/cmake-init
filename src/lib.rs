//! Library backing the `cmake-init` command-line tool.
//!
//! It scaffolds a CMake C++ project: generating sources, a `CMakeLists.txt`,
//! and optional `.gitignore` and Git repository. The modules cover each
//! generated artifact, while this root provides the shared CLI input
//! validators [`parse_bool_flag`] and [`validate_project_name`].

pub mod _gitignore;
pub mod cmakelists;
pub mod features;
pub mod git;
pub mod sources;

/// Parses a boolean flag value, accepting common truthy/falsy spellings
/// case-insensitively: `true`/`false`, `yes`/`no`, `on`/`off`, `1`/`0`.
pub fn parse_bool_flag(s: &str) -> Result<bool, String> {
    match s.to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" => Ok(true),
        "false" | "no" | "off" | "0" => Ok(false),
        _ => Err(format!(
            "invalid boolean value '{s}': expected true/false, yes/no, on/off, or 1/0"
        )),
    }
}

/// Returns `Ok(())` when `name` is a valid CMake project identifier.
///
/// Accepted characters: ASCII letters (`A–Z`, `a–z`), digits (`0–9`),
/// hyphens (`-`), and underscores (`_`).  Spaces, parentheses, semicolons,
/// quotes, and other shell/CMake special characters are rejected because
/// CMake parses the `project(...)` argument list as whitespace-separated
/// tokens and treats several punctuation characters as syntax.
pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name must not be empty.".to_string());
    }
    if name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        Ok(())
    } else {
        Err(format!(
            "Invalid project name '{name}'. \
             Accepted characters: ASCII letters, digits, hyphens (-), and underscores (_)."
        ))
    }
}

#[cfg(test)]
mod bool_flag_tests {
    use super::parse_bool_flag;

    #[test]
    fn accepts_true_variants() {
        assert_eq!(parse_bool_flag("true"), Ok(true));
        assert_eq!(parse_bool_flag("True"), Ok(true));
        assert_eq!(parse_bool_flag("TRUE"), Ok(true));
        assert_eq!(parse_bool_flag("yes"), Ok(true));
        assert_eq!(parse_bool_flag("Yes"), Ok(true));
        assert_eq!(parse_bool_flag("on"), Ok(true));
        assert_eq!(parse_bool_flag("1"), Ok(true));
    }

    #[test]
    fn accepts_false_variants() {
        assert_eq!(parse_bool_flag("false"), Ok(false));
        assert_eq!(parse_bool_flag("False"), Ok(false));
        assert_eq!(parse_bool_flag("FALSE"), Ok(false));
        assert_eq!(parse_bool_flag("no"), Ok(false));
        assert_eq!(parse_bool_flag("No"), Ok(false));
        assert_eq!(parse_bool_flag("off"), Ok(false));
        assert_eq!(parse_bool_flag("0"), Ok(false));
    }

    #[test]
    fn rejects_unrecognised_values() {
        assert!(parse_bool_flag("maybe").is_err());
        assert!(parse_bool_flag("2").is_err());
        assert!(parse_bool_flag("").is_err());
        assert!(parse_bool_flag("enabled").is_err());
    }
}

#[cfg(test)]
mod name_validation_tests {
    use super::validate_project_name;

    #[test]
    fn valid_names_are_accepted() {
        assert!(validate_project_name("myproject").is_ok());
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("MyProject42").is_ok());
        assert!(validate_project_name("_internal").is_ok());
        assert!(validate_project_name("a-b_c-D3").is_ok());
    }

    #[test]
    fn space_is_rejected() {
        assert!(validate_project_name("my project").is_err());
    }

    #[test]
    fn open_paren_is_rejected() {
        assert!(validate_project_name("proj(x)").is_err());
    }

    #[test]
    fn semicolon_is_rejected() {
        assert!(validate_project_name("proj;bad").is_err());
    }

    #[test]
    fn empty_name_is_rejected() {
        assert!(validate_project_name("").is_err());
    }
}

#[cfg(test)]
pub mod test_util {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Mutex, OnceLock};

    /// Serializes tests that mutate the process-wide current directory so they
    /// do not race with one another when the test harness runs them in
    /// parallel.
    static CWD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    /// Guarantees a unique temp directory name per call within this process.
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Runs `f` with the process current directory set to a fresh, empty
    /// temporary directory. The original working directory is restored and the
    /// temporary directory removed afterwards, even if `f` panics, so tests
    /// leave no files behind in the repository.
    pub fn in_temp_dir<F: FnOnce()>(f: F) {
        let guard = CWD_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            // Recover from a poisoned lock: a previous test panicking should not
            // prevent the remaining tests from running.
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let original = std::env::current_dir().expect("failed to read current dir");

        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "cmake-init-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir_all(&dir).expect("failed to create temp dir");
        std::env::set_current_dir(&dir).expect("failed to enter temp dir");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

        // Always restore state, regardless of whether `f` succeeded.
        std::env::set_current_dir(&original).expect("failed to restore current dir");
        let _ = std::fs::remove_dir_all(&dir);

        drop(guard);

        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }
}
