use std::num::NonZeroU32;

#[no_mangle]
pub extern "C" fn unwrapped(value: NonZeroU32, opt: Option<NonZeroU32>) {}

#[repr(transparent)]
pub struct Wrapped {
    value: NonZeroU32,
}

#[no_mangle]
pub extern "C" fn wrapped(value: Wrapped, opt: Option<Wrapped>) {}