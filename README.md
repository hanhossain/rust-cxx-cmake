# Example project to integrate Rust and C++ together with cmake

```mermaid
graph TD
common-rs --> middle-rs
middle-rs --> runner-rs

runner-cpp

```

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
