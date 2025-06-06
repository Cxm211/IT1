#include <stddef.h>

struct Foo {
    size_t data;
};

struct Bar {
    size_t data;
    ~Bar() { data = 0; }
};

Foo MakeFoo(); // { return Foo(); }

Bar MakeBar(); // { return Bar(); }