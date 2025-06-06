pub const SIZE: usize = 4;

#[repr(C)]
pub struct WithoutAs {
    items: [char; SIZE],
}

#[repr(C)]
pub struct WithAs {
    items: [char; SIZE as usize],
}

// dummy function to make `WithoutAs` and `WithAs` part of the public api
#[no_mangle]
pub extern fn some_fn(a: WithoutAs, b: WithAs) {}