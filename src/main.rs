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
                .short('n')
                .long("project-name")
                .default_value("project")
                .help("Project name"),
        )
        .arg(
            Arg::new("cxx-std")
                .short('s')
                .long("cxx-std")
                .default_value("17")
                .help("C++ standard (11, 14, 17, 20, 23)"),
        )
        .arg(
            Arg::new("gitignore")
                .short('g')
                .long("gitignore")
                .default_value("true")
                .help("Configure .gitignore"),
        )
        .get_matches();

    let project_name = matches.get_one::<String>("project-name").unwrap();
    println!(
        "Current path: {}",
        std::env::current_dir().unwrap().display()
    );

    // Create project directory and set current path
    fs::create_dir_all(project_name).expect("Failed to create project directory");
    std::env::set_current_dir(project_name).expect("Failed to change directory");

    let cxx_std: i32 = matches
        .get_one::<String>("cxx-std")
        .unwrap()
        .parse()
        .expect("Invalid C++ standard");

    if ![11, 14, 17, 20, 23].contains(&cxx_std) {
        eprintln!("Invalid C++ standard. Please select 11, 14, 17, 20, or 23.");
        return;
    }

    src_main_cpp(); // External function for creating src/main.cpp
    cmakelists_txt(project_name, cxx_std); // External function for creating CMakeLists.txt

    let configure_gitignore = matches.get_one::<String>("gitignore").unwrap() == "true";
    if configure_gitignore {
        gitignore();
    }

    println!("All done!");
}