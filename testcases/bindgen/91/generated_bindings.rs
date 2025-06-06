/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    _unused: [u8; 0],
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Copy, Clone)]
pub struct B {
    pub _bindgen_opaque_blob: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 1usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}__ZN1B5m_fn1Ev"]
    pub fn B_m_fn1(this: *mut B) -> A;
}
impl B {
    #[inline]
    pub unsafe fn m_fn1(&mut self) -> A {
        B_m_fn1(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub some_member: B,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 1usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 1usize];
    ["Offset of field: C::some_member"][::std::mem::offset_of!(C, some_member) - 0usize];
};
