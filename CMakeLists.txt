cmake_minimum_required(VERSION 3.30)
project(rust-cxx-cmake LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

option(CALLED_FROM_RUST "called from rust" OFF)

if (NOT CALLED_FROM_RUST)
    include_directories(target/cxxbridge)
endif ()

add_subdirectory(middle-cpp)

install(TARGETS middle-cpp)