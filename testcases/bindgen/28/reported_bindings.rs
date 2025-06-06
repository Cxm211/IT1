#[repr(C)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}

// ...

#[repr(C, packed)]
pub struct Date {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
}