
bitflags! {
    #[repr(C)]
    pub struct LargeFlags: u64 {
        /// Flag with a very large shift that usually would be narrowed.
        const LARGE_SHIFT = 1u64 << 44;
        const INVERTED = !Self::LARGE_SHIFT.bits();
    }
}


#[no_mangle]
pub extern "C" fn root(

    largest_flags: LargeFlags,

) {
}