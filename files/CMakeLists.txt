cmake_minimum_required(VERSION 3.20.0)
project({{PROJECT_NAME}} VERSION 0.0.0)

set(CMAKE_CXX_STANDARD {{cxx_std}})

file(GLOB_RECURSE SRC_FILES ./src/*.cpp)

add_executable(${PROJECT_NAME} ${SRC_FILES})

install(TARGETS ${PROJECT_NAME})
