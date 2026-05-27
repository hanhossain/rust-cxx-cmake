#pragma once

#include <memory>
#include <string>

class MiddleCpp
{
public:
    MiddleCpp(std::string owner);
    const std::string& print() const;

private:
    std::string owner_;
};

std::unique_ptr<MiddleCpp> MiddleCpp_new(const std::string& owner);
