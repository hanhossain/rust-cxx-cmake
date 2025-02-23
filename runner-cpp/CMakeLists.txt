add_executable(runner-cpp main.cpp)

target_link_libraries(runner-cpp PRIVATE middle-cpp middle_rs)
target_include_directories(runner-cpp PRIVATE ${CXXBRIDGE_PATH})
