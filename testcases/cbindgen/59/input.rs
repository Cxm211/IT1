struct Foo {
    data: *const Bar,
}
struct Bar {
    data: *const Foo,
}
#[no_mangle]
pub extern "C" fn root(
    x: Bar,
    y: Foo,
) {}