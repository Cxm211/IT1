/* automatically generated by rust-bindgen 0.71.1 */

unsafe extern "C" {
    #[link_name = "\u{1}__Z1fiz"]
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
            ...
        ),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 8usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 8usize];
    ["Offset of field: Foo::f"][::std::mem::offset_of!(Foo, f) - 0usize];
};
