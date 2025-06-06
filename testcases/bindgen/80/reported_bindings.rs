// ... stripped

extern crate core;

#[repr(C)]
#[derive(Copy)]
pub struct C {
    pub _bitfield_1: u8,
    pub large_array: [::std::os::raw::c_int; 50usize]
}

// stripped ...