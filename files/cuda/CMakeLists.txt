cmake_minimum_required(VERSION 3.20.0)
project({{PROJECT_NAME}} VERSION 0.0.0 LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD {{cxx_std}})

set(CMAKE_CUDA_ARCHITECTURES 80)

set(CUDA_COMPILER /usr/bin/nvcc)
set(CMAKE_CUDA_COMPILER /usr/bin/nvcc)

find_package(CUDA REQUIRED)

file(GLOB_RECURSE SRC_FILES ./src/*.cu)

add_executable(${PROJECT_NAME} ${SRC_FILES})

install(TARGETS ${PROJECT_NAME})

set_source_files_properties(
    src/main.cu
    PROPERTIES LANGUAGE CUDA
)
