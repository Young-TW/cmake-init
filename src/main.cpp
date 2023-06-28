#include <iostream>
#include <filesystem>
#include <fstream>

#include <cxxopts.hpp>

#include "files/CMakeLists_txt.hpp"
#include "files/src_main_cpp.hpp"
#include "files/_gitignore.hpp"

int main(int argc, char* argv[]) {
    cxxopts::Options options("cmake-init", "A simple CMake project initializer");

    options.add_options()
        ("h,help", "Show this help message and exit", cxxopts::value<bool>()->default_value("false"))
        ("v,version", "Show version and exit", cxxopts::value<bool>()->default_value("false"))
        ("n,project-name", "Project name", cxxopts::value<std::string>()->default_value("project"))
        ("s,cxx-std", "C++ standard(11, 14, 17, 20, 23)", cxxopts::value<int>()->default_value("17"))
        ("g,gitignore", "Configure .gitignore", cxxopts::value<bool>()->default_value("true"));

    auto result = options.parse(argc, argv);

    if (result.count("help")) {
        std::cout << options.help() << std::endl;
        return 0;
    }

    if (result.count("version")) {
        std::cout << "0.1.0" << std::endl;
        return 0;
    }

    std::string project_name;
    if (result.count("project-name") != 0) {
        project_name = result["project-name"].as<std::string>();
    } else {
        std::cout << "Project name: ";
        std::cin >> project_name;
    }

    std::filesystem::create_directory(project_name);
    std::filesystem::path project_root = std::filesystem::current_path() / project_name;

    int cxx_std;

    if (result.count("cxx-std")) {
        cxx_std = result["cxx-std"].as<int>();
    } else {
        do {
            std::cout << "Select C++ standard(11, 14, 17, 20, 23): ";
            std::cin >> cxx_std;
        } while (cxx_std != 11 && cxx_std != 14 && cxx_std != 17 && cxx_std != 20 && cxx_std != 23);
    }

    std::filesystem::create_directory("/src");
    src_main_cpp();

    if (result.count("gitignore")) {
        if (result["gitignore"].as<bool>()) {
            gitignore();
        }
    } else {
        std::cout << "Configure .gitignore(Y/n): ";
        bool gitignore_config = true;
        char status;
        std::cin >> status;
        if(status == 'n' || status == 'N') {
            gitignore_config = false;
        }

        if (gitignore_config) {
            gitignore();
        }
    }

    std::cout << "All done!";

    return 0;
}
