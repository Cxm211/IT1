#[repr(C, packed(2))]
#[repr(align(2))]
#[derive(Debug, Copy, Clone)]
pub struct FndrOpaqueInfo {
    pub opaque: [::std::os::raw::c_char; 16usize],
}