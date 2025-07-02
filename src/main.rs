use clap::{Arg, ArgAction, Command};
use std::fs;

use cmake_init::_gitignore::gitignore;
use cmake_init::cmakelists_txt::cmakelists_txt;
use cmake_init::src_main_cpp::src_main_cpp;

fn main() {
    let matches = Command::new("cmake-init")
        .version("0.1.0")
        .about("A simple CMake project initializer")
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::Help),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version),
        )
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
        .get_matches();

    let project_name = matches.get_one::<String>("project-name").unwrap();

    // Create project directory and set current path
    fs::create_dir_all(project_name).expect("Failed to create project directory");
    std::env::set_current_dir(project_name).expect("Failed to change directory");

    let cxx_std: i32 = matches
        .get_one::<String>("cxx-std")
        .unwrap()
        .parse()
        .expect("Invalid C++ standard");

    if ![11, 14, 17, 20, 23, 26].contains(&cxx_std) {
        eprintln!("Invalid C++ standard. Please select 11, 14, 17, 20, 23, or 26.");
        return;
    }

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

    println!("All done!");
}
