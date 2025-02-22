cmake_minimum_required(VERSION 3.31)
project(rust-cxx-cmake LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(runner-cpp)
add_subdirectory(middle-cpp)
