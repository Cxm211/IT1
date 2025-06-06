// nested_template.h
#include <vector>

template <typename T>
class Wrapper {
    T value;
};

template <typename T>
class NestedWrapper {
    Wrapper<T> inner;
};

using IntNestedWrapper = NestedWrapper<int>;
using VectorNestedWrapper = NestedWrapper<std::vector<int>>;
