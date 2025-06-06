#include <vector>
template <typename T>
class SpecialWrapper {
    T value;
};

template <typename T>
class NestedSpecialWrapper {
    SpecialWrapper<T> inner;
};

using IntSpecialWrapper = NestedSpecialWrapper<int>;
using VectorSpecialWrapper = NestedSpecialWrapper<std::vector<int>>;