#pragma once

#include <string>

class MiddleCpp
{
public:
    MiddleCpp(std::string owner);
    std::string print() const;

private:
    std::string owner_;
};
