#[repr(C)]
pub struct Test {
    pub data: [u8; Self::LEN],
}

impl Test {
    pub const LEN: usize = 8;
}

#[no_mangle]
pub extern fn hello(test: &mut Test) {
    println!("hello");
}
