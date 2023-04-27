#include <iostream>
#include <filesystem>
#include <fstream>

#include "files/CMakeLists_txt.h"
#include "files/src_main_cpp.h"
#include "files/_gitignore.h"

int main(int argc, char* argv[]) {
    std::cout << "Project name: ";
    std::string project_name;
    std::cin >> project_name;

    int cxx_std;
    do {
        std::cout << "Select C++ standard(11, 14, 17, 20, 23): ";
        std::cin >> cxx_std;
    } while (cxx_std != 11 && cxx_std != 14 && cxx_std != 17 && cxx_std != 20 && cxx_std != 23);

    std::cout << "Configure .gitignore(Y/n): ";
    bool gitignore_config = true;
    char status;
    std::cin >> status;
    if(status == 'n' || status == 'N') {
        gitignore_config = false;
    }

    if (argc == 1) {
        std::filesystem::create_directory("src");
        std::filesystem::create_directory("include");
        cmakelists_txt(project_name, cxx_std);
        src_main_cpp();
        if(gitignore_config) {
            gitignore();
        }
    } else if (argc == 2) {
        if (!std::filesystem::exists(argv[1])) {
            std::filesystem::create_directory(argv[1]);
        }
        std::filesystem::current_path(argv[1]);
        std::filesystem::create_directory("src");
        std::filesystem::create_directory("include");
        cmakelists_txt(project_name, cxx_std);
        src_main_cpp();
        if(gitignore_config) {
            gitignore();
        }
    }

    return 0;
}
