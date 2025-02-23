#include "middle-rs/src/lib.rs.h"
#include "middle_cpp.h"
#include <iostream>

int main()
{
    std::cout << "Hello world from runner-cpp!" << std::endl;
    middle_cpp::MiddleCpp middle("runner-cpp");
    middle.print();

    auto middle_rs = middle_rs::Middle_new("runner-cpp", common_rs::Language::Cpp);
    middle_rs->run();
    return 0;
}