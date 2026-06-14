pub mod _gitignore;
pub mod cmakelists_txt;
pub mod features;
pub mod git;
pub mod src_main_cpp;

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
