add_library(middle-cpp middle_cpp.cpp)
# link to common_rs to prevent compilation before common_rs is ready
target_link_libraries(middle-cpp PRIVATE common_rs)
target_include_directories(middle-cpp
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
    PRIVATE ${CXXBRIDGE_PATH}
)
