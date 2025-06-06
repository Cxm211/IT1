use block;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct contains_ptr_to_block_ptr {
    pub val: _bindgen_ty_id_4,
}
#[test]
fn bindgen_test_layout_contains_ptr_to_block_ptr() {
    assert_eq!(
        ::std::mem::size_of::<contains_ptr_to_block_ptr>(),
        8usize,
        concat!("Size of: ", stringify!(contains_ptr_to_block_ptr))
    );
    assert_eq!(
        ::std::mem::align_of::<contains_ptr_to_block_ptr>(),
        8usize,
        concat!("Alignment of ", stringify!(contains_ptr_to_block_ptr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<contains_ptr_to_block_ptr>())).val
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(contains_ptr_to_block_ptr),
            "::",
            stringify!(val)
        )
    );
}
pub type _bindgen_ty_id_4 = *const ::block::Block<(::std::os::raw::c_int,), ()>;