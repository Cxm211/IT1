extern "C" {
    pub fn f(a: ::std::os::raw::c_int, ...);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub f: ::std::option::Option<
        unsafe extern "C" fn(
            p: *mut ::std::os::raw::c_void,
            obj: *mut ::std::os::raw::c_void,
            a: ::std::os::raw::c_int,
        ),
    >,
}