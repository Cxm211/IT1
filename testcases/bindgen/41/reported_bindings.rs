#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union TestUnion {
    pub Longs: [::std::os::raw::c_ulong; 2usize],
    pub Ints: [::std::os::raw::c_uint; 4usize],
    _bindgen_union_align: u128,
}