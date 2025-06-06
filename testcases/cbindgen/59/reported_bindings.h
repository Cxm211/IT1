struct Bar;
struct Foo {
    const Bar* data;
}
struct Bar {
    const Foo* data;
}