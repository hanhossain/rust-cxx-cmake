# Integrate Rust and C++ together with cmake

## Architecture

```mermaid
graph TD
    common-rs --> middle-rs
    common-rs --> middle-cpp
    middle-rs --> runner-rs
    middle-rs --> runner-cpp
    middle-cpp --> runner-cpp
    middle-cpp --> runner-rs
```

## TODO

- common-rs
    - Rust struct (with a method)
- common-cpp
    - Class with vec member
        - With method
- hidden-rs
    - Rust struct (with a method)
        - Uses common-cpp
- middle-cpp
    - Uses hidden-rs

## Building and running

### Rust

```shell
cargo run
```

### C++

```shell
cmake -B build -G Ninja
cmake --build build
./build/runner-cpp/runner-cpp
```
