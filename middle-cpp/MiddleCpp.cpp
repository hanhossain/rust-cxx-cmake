#include "middle-cpp/MiddleCpp.h"

#include <iostream>
#include <utility>

MiddleCpp::MiddleCpp(std::string owner)
    : owner_(std::move(owner))
{
    std::cout << "[MiddleCpp] Creating MiddleCpp" << std::endl;
}

const std::string& MiddleCpp::print() const
{
    std::cout << "[MiddleCpp::print] Owner is " << owner_ << std::endl;
    return owner_;
}

std::unique_ptr<MiddleCpp> MiddleCpp_new(const std::string& owner)
{
    return std::make_unique<MiddleCpp>(owner);
}
