cmake_minimum_required(VERSION 3.30)
project(rust-cxx-cmake LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory(middle-cpp)

install(TARGETS middle-cpp)