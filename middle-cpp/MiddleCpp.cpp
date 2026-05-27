#include "middle-cpp/MiddleCpp.h"

#include <iostream>
#include <utility>

#include "rust/cxx.h"
#include "middle-rs/src/lib.rs.h"

MiddleCpp::MiddleCpp(std::string owner, rust::Box<SomethingOpaque> something_opaque)
    : owner_(std::move(owner)),
      something_opaque_(std::move(something_opaque))
{
    std::cout << "[MiddleCpp] Creating MiddleCpp" << std::endl;
}

const std::string& MiddleCpp::print() const
{
    std::cout << "[MiddleCpp::print] Owner is " << owner_ << std::endl;
    something_opaque_->print();
    return owner_;
}

void MiddleCpp::change_owner()
{
    std::cout << "[MiddleCpp::change_owner] Changing owner" << std::endl;
    something_opaque_->set_owner("MiddleCpp");
}

std::unique_ptr<MiddleCpp> MiddleCpp_new(const std::string& owner, rust::Box<SomethingOpaque> something_opaque)
{
    return std::make_unique<MiddleCpp>(owner, std::move(something_opaque));
}
