use clap::{Arg, ArgAction, Command};
use std::fs;

use cmake_init::_gitignore::gitignore;
use cmake_init::cmakelists_txt::cmakelists_txt;
use cmake_init::git::git_init;
use cmake_init::src_main_cpp::src_main_cpp;

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
                .help("Configure .gitignore"),
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
                .help("Initialize Git repository (`true`/`false`)"),
        )
        .get_matches();

    let project_name = matches.get_one::<String>("project-name").unwrap();

    // Validate inputs before creating anything on disk so a bad value does
    // not leave a stray, empty project directory behind.
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

    let mode = if matches.get_flag("cuda") {
        "CUDA"
    } else if matches.get_flag("hip") {
        "HIP"
    } else if matches.get_flag("mpi") {
        "MPI"
    } else {
        "C++"
    };

    match mode {
        "CUDA" => {
            src_main_cpp(Some("CUDA"));
            cmakelists_txt(project_name, cxx_std, Some("CUDA"));
            println!("CUDA support enabled.");
        }
        "HIP" => {
            src_main_cpp(Some("HIP"));
            cmakelists_txt(project_name, cxx_std, Some("HIP"));
            println!("HIP support enabled.");
        }
        "MPI" => {
            src_main_cpp(Some("MPI"));
            cmakelists_txt(project_name, cxx_std, Some("MPI"));
            println!("OpenMPI support enabled.");
        }
        _ => {
            src_main_cpp(Some("C++"));
            cmakelists_txt(project_name, cxx_std, Some("C++"));
        }
    }

    let configure_gitignore = matches.get_one::<String>("gitignore").unwrap() == "true";
    if configure_gitignore {
        gitignore();
    }

    let configure_git = matches.get_one::<String>("git").unwrap() == "true";
    if configure_git {
        git_init();
    }

    println!("All done!");
}
