#include "middle_cpp.h"
#include <iostream>

middle_cpp::MiddleCpp::MiddleCpp(std::string caller)
    : _caller(caller)
{
}

void middle_cpp::MiddleCpp::print()
{
    std::cout << "[" << _caller << "] middle_cpp::MiddleCpp::print()" << std::endl;
}
