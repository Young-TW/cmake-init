#ifndef FILES_CMAKELIST_TXT_HPP
#define FILES_CMAKELIST_TXT_HPP

#include <string>

#include "fstream.h"

int cmakelists_txt(std::string project_name, int cxx_std){
    fout.open("CMakeLists.txt");
    fout << "cmake_minimum_required(VERSION 3.20.0)\n"
    << "project(" << project_name << " VERSION 0.0.0)\n\n"
    << "set(CMAKE_CXX_STANDARD " << cxx_std << ")\n\n"
    << "include_directories(./include)\n\n"
    << "file(GLOB_RECURSE SRC_FILES ./src/*.cpp)\n\n"
    << "add_executable(${PROJECT_NAME} ${SRC_FILES})";
    fout.close();
    return 0;
}

#endif