//  C++ code
struct Foo {
    char a;
    char b __attribute__((aligned(8)));
    char c[];
};