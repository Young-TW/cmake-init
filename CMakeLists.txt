cmake_minimum_required(VERSION 3.20.0)
project(cmake-init VERSION 0.0.0)

set(CMAKE_CXX_STANDARD 23)

include_directories(./include)

add_subdirectory(cxxopts)

file(GLOB_RECURSE SRC_FILES ./src/*.cpp)

add_executable(${PROJECT_NAME} ${SRC_FILES})
target_link_libraries(${PROJECT_NAME} PRIVATE cxxopts)

install(TARGETS cmake-init)