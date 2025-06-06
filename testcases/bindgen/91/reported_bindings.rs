#[repr(C)]
#[derive(Debug, Copy)]
pub struct B {
    pub _bindgen_opaque_blob: u8,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(::std::mem::size_of::<B>() , 1usize , concat ! (
               "Size of: " , stringify ! ( B ) ));
    assert_eq! (::std::mem::align_of::<B>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( B ) ));
}
extern "C" {
    #[link_name = "_ZN1B5m_fn1Ev"]
    pub fn B_m_fn1(this: *mut B) -> A;
}
impl Clone for B {
    fn clone(&self) -> Self { *self }
}
impl B {
    #[inline]
    pub unsafe fn m_fn1(&mut self) -> A { B_m_fn1(self) }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub some_type: B,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 1usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . some_type as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( some_type ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
