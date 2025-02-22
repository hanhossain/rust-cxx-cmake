#include <string>

namespace middle_cpp
{
    class MiddleCpp
    {
    public:
        MiddleCpp(std::string caller);
        void print();

    private:
        std::string _caller;
    };
}
