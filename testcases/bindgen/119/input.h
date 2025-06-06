template <template <typename> class Bar>
struct Foo {
private:
    static Bar<unsigned> bar;
};