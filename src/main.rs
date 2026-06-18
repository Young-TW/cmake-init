//! `cmake-init` command-line entry point.
//!
//! Parses CLI arguments, validates them, creates a project directory named
//! after the project, and populates it with generated sources, a
//! `CMakeLists.txt`, and optionally a `.gitignore` and an initialized Git
//! repository.

use clap::{Arg, ArgAction, Command};
use std::fs;
use std::fs::File;
use std::io::Write;

use cmake_init::_gitignore::gitignore;
use cmake_init::cmakelists;
use cmake_init::features::{Backend, Features};
use cmake_init::git::git_init;
use cmake_init::parse_bool_flag;
use cmake_init::sources::write_sources;

fn main() {
    let matches = Command::new("cmake-init")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple CMake project initializer")
        .arg(
            Arg::new("project-name")
                .index(1)
                .required(true)
                .help("Project name"),
        )
        .arg(
            Arg::new("cxx-std")
                .short('s')
                .long("cxx-std")
                .default_value("17")
                .help("C++ standard (11, 14, 17, 20, 23, 26)"),
        )
        .arg(
            Arg::new("gitignore")
                .short('g')
                .long("gitignore")
                .default_value("true")
                .value_parser(parse_bool_flag)
                .help("Configure .gitignore (true/false/yes/no/on/off/1/0)"),
        )
        .arg(
            Arg::new("cuda")
                .short('c')
                .long("cuda")
                .action(ArgAction::SetTrue)
                .help("Enable CUDA support"),
        )
        .arg(
            Arg::new("hip")
                .short('i')
                .long("hip")
                .action(ArgAction::SetTrue)
                .help("Enable HIP support"),
        )
        .arg(
            Arg::new("mpi")
                .short('m')
                .long("mpi")
                .action(ArgAction::SetTrue)
                .help("Enable OpenMPI support"),
        )
        .arg(
            Arg::new("git")
                .long("git")
                .default_value("true")
                .value_parser(parse_bool_flag)
                .help("Initialize Git repository (true/false/yes/no/on/off/1/0)"),
        )
        .get_matches();

    let project_name = matches.get_one::<String>("project-name").unwrap();

    // Validate inputs before creating anything on disk so a bad value does
    // not leave a stray, empty project directory behind.
    if let Err(msg) = cmake_init::validate_project_name(project_name) {
        eprintln!("{msg}");
        std::process::exit(1);
    }

    let cxx_std_raw = matches.get_one::<String>("cxx-std").unwrap();
    let cxx_std: i32 = match cxx_std_raw.parse() {
        Ok(value) if [11, 14, 17, 20, 23, 26].contains(&value) => value,
        _ => {
            eprintln!(
                "Invalid C++ standard '{cxx_std_raw}'. Please select 11, 14, 17, 20, 23, or 26."
            );
            std::process::exit(1);
        }
    };

    // Create project directory and set current path
    fs::create_dir_all(project_name).expect("Failed to create project directory");
    std::env::set_current_dir(project_name).expect("Failed to change directory");

    // Collect backends in canonical order (CUDA before HIP) so derived target
    // names and emitted CMake sections stay stable.
    let mut backends = Vec::new();
    if matches.get_flag("cuda") {
        backends.push(Backend::Cuda);
    }
    if matches.get_flag("hip") {
        backends.push(Backend::Hip);
    }
    let features = Features {
        mpi: matches.get_flag("mpi"),
        backends,
    };

    write_sources(&features);

    let cmakelists = cmakelists::render(project_name, cxx_std, &features);
    File::create("CMakeLists.txt")
        .and_then(|mut file| file.write_all(cmakelists.as_bytes()))
        .expect("Failed to write CMakeLists.txt");

    if features.has(Backend::Cuda) {
        println!("CUDA support enabled.");
    }
    if features.has(Backend::Hip) {
        println!("HIP support enabled.");
    }
    if features.mpi {
        println!("OpenMPI support enabled.");
    }

    if *matches.get_one::<bool>("gitignore").unwrap() {
        gitignore();
    }

    if *matches.get_one::<bool>("git").unwrap() {
        git_init();
    }

    println!("All done!");
}
