#include <string>

namespace middle_cpp
{
    class MiddleCpp
    {
    public:
        MiddleCpp(std::string caller);
        void print() const;

    private:
        std::string _caller;
    };

    std::unique_ptr<MiddleCpp> MiddleCpp_new(const std::string &caller);
}
