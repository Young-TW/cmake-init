cmake_minimum_required(VERSION 3.20.0)
project({{PROJECT_NAME}} VERSION 0.0.0)

set(CMAKE_CXX_STANDARD {{cxx_std}})

set(HIP_COMPILER clang)
set(CMAKE_HIP_COMPILER /opt/rocm/bin/hipcc)

find_package(HIP REQUIRED)

file(GLOB_RECURSE SRC_FILES ./src/*.hip)

add_executable(${PROJECT_NAME} ${SRC_FILES})

set_source_files_properties(src/main.hip PROPERTIES LANGUAGE HIP)

install(TARGETS ${PROJECT_NAME})
