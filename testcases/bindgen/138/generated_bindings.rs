/* automatically generated by rust-bindgen 0.71.1 */

pub type std_string = ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std_allocator_traits___size_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Vector_base {
    pub _Tp_alloc_type: __gnu_cxx___alloc_traits,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_vector {
    pub _base: std__Vector_base,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_LogMessage {
    pub _address: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct std_LogMessage_ostrstream {
    pub __bindgen_anon_1: std_LogMessage_ostrstream__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union std_LogMessage_ostrstream__bindgen_ty_1 {
    pub outvec_: *mut std_vector,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits {
    pub _address: u8,
}
