use std::process::Command;

pub fn git_init() {
    let output = Command::new("git")
        .arg("init")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Initialized empty Git repository.");
        }
        Ok(output) => {
            eprintln!("Failed to initialize Git repository: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Failed to execute git init: {}. Is git installed?", e);
        }
    }
}
