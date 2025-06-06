/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy)]
pub struct example {
    pub field1: example_struct1,
}
#[test]
fn bindgen_test_layout_example() {
    assert_eq!(::std::mem::size_of::<example>() , 8usize , concat ! (
               "Size of: " , stringify ! ( example ) ));
    assert_eq! (::std::mem::align_of::<example>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( example ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const example ) ) . field1 as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( example ) , "::" ,
                stringify ! ( field1 ) ));
}
impl Clone for example {
    fn clone(&self) -> Self { *self }
}