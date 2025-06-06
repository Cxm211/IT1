// crate/src/lib.rs
mod Foo {
    #[repr(C)]
    struct Bar {}
 
    #[no_mangle]
    extern "C" fn root(a: Bar) { }
 }