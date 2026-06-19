//! Regression test for GitHub issue #6:
//! "partial project directory not cleaned up when generation fails mid-way".
//!
//! `main()` creates the project directory (and `cd`s into it) before all
//! file-writing work is done. If a later step fails, the half-written project
//! directory must not be left behind on disk — and because *this run* created
//! it, the whole directory should be removed (acceptance criteria bullet 1).
//!
//! This test drives the real binary so it exercises the actual orchestration in
//! `main.rs`. It forces a mid-way write failure by lowering the umask so the
//! freshly created project directory is not writable: `create_dir_all` and the
//! `cd` succeed, but the subsequent `write_sources` step (which creates
//! `./src`) fails. The expected behaviour is that the stub project directory is
//! gone afterwards.

// umask-based failure injection is Unix-specific.
#![cfg(unix)]

use std::path::PathBuf;
use std::process::Command;

/// Create a fresh, uniquely named empty temp directory and return its path.
fn fresh_temp_dir() -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "cmake-init-issue6-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn failed_generation_removes_its_stub_directory() {
    let work_dir = fresh_temp_dir();
    let project_name = "stubproj";
    let project_dir = work_dir.join(project_name);

    // Run the binary from inside `work_dir` with a umask that strips the write
    // bit from any directory it creates. `create_dir_all(project_name)` still
    // succeeds (only the parent needs to be writable), the process `cd`s in,
    // but creating `./src` inside the now-read-only project directory fails,
    // aborting generation partway through.
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "umask 0222; exec '{}' {}",
            env!("CARGO_BIN_EXE_cmake-init"),
            project_name
        ))
        .current_dir(&work_dir)
        .status()
        .expect("failed to spawn cmake-init");

    let stub_exists = project_dir.exists();

    // Tidy up before asserting so a failing assertion still leaves no garbage.
    let _ = std::fs::remove_dir_all(&work_dir);

    // The generation step was meant to fail (that is how we provoke the bug).
    assert!(
        !status.success(),
        "expected cmake-init to fail when its project directory is not writable, \
         but it exited successfully"
    );

    // Expected (correct) behaviour: the directory this run created must not be
    // left behind as a stub after the failure.
    assert!(
        !stub_exists,
        "issue #6: project directory '{}' created by the failed run was not cleaned up",
        project_dir.display()
    );
}
