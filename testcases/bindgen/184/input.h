// input.h
template<typename T, typename... U> union Foo {
  Foo<U...> foo;
};