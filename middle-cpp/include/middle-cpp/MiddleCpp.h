#pragma once

#include <memory>
#include <string>

#include "rust/cxx.h"

struct SomethingOpaque;

class MiddleCpp
{
public:
    MiddleCpp(std::string owner, rust::Box<SomethingOpaque> something_opaque);
    const std::string& print() const;
    void change_owner();

private:
    std::string owner_;
    rust::Box<SomethingOpaque> something_opaque_;
};

std::unique_ptr<MiddleCpp> MiddleCpp_new(const std::string& owner, rust::Box<SomethingOpaque> something_opaque);
