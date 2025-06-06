const LENGTH: usize = 127;

#[repr(C)]
pub struct Data {
    pub d: [u8; LENGTH + 1],
}