cmake_minimum_required(VERSION 3.30)
project(rust-cxx-cmake LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    CRATE_TYPES staticlib
)

set(CMAKE_OSX_DEPLOYMENT_TARGET 15.1)

set(CXXBRIDGE_PATH ${CMAKE_CURRENT_BINARY_DIR}/cargo/build/${Rust_CARGO_TARGET_CACHED}/cxxbridge)

add_subdirectory(runner-cpp)
add_subdirectory(middle-cpp)
