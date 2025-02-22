#include "middle_cpp.h"
#include <iostream>

middle_cpp::MiddleCpp::MiddleCpp(std::string caller)
    : _caller(caller)
{
}

void middle_cpp::MiddleCpp::print() const
{
    std::cout << "[" << _caller << "] middle_cpp::MiddleCpp::print()" << std::endl;
}

std::unique_ptr<middle_cpp::MiddleCpp> middle_cpp::MiddleCpp_new(const std::string &caller)
{
    return std::make_unique<middle_cpp::MiddleCpp>(caller);
}
