template <typename T>
struct Inner {
  T foo;
};

template <typename T>
struct Outer {
  Inner<T[1]> inner;
};