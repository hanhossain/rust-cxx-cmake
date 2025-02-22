#include "middle_cpp.h"
#include <iostream>

int main()
{
    std::cout << "Hello world from runner-cpp!" << std::endl;
    middle_cpp::MiddleCpp middle("runner-cpp");
    middle.print();
    return 0;
}