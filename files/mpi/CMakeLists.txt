cmake_minimum_required(VERSION 3.20.0)
project({{PROJECT_NAME}} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD {{cxx_std}})
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(MPI REQUIRED)

file(GLOB_RECURSE SRC_FILES ./src/*.cpp)

add_executable(${PROJECT_NAME} ${SRC_FILES})

target_link_libraries(${PROJECT_NAME} MPI::MPI_CXX)
