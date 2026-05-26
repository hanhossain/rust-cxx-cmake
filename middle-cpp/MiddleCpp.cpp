#include "MiddleCpp.h"

#include <iostream>
#include <utility>

MiddleCpp::MiddleCpp(std::string owner)
    : owner_(std::move(owner))
{
    std::cout << "[MiddleCpp] Creating MiddleCpp" << std::endl;
}

std::string MiddleCpp::print() const
{
    std::cout << "[MiddleCpp::print] Owner is " << owner_ << std::endl;
    return owner_;
}
