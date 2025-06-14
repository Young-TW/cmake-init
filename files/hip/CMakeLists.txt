cmake_minimum_required(VERSION 3.20.0)
project({{PROJECT_NAME}} VERSION 0.0.0)

set(CMAKE_CXX_STANDARD {{cxx_std}})

include_directories(./include)

set(HIP_COMPILER clang)
set(CMAKE_HIP_COMPILER /opt/rocm/bin/hipcc)

find_package(HIP REQUIRED)

add_executable(${PROJECT_NAME} src/main.cpp)

set_source_files_properties(src/main.cpp PROPERTIES LANGUAGE HIP)

install(TARGETS ${PROJECT_NAME})
