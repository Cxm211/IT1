// lib.rs

#[cfg(foo)]
pub const DOES_WORK: i32 = 1;

#[cfg(bar)]
pub const DOES_WORK: i32 = 2;

#[cfg(foo)]
#[no_mangle]
pub unsafe extern "C" fn does_work(dw: &DoesWork) {}

#[cfg(bar)]
#[no_mangle]
pub unsafe extern "C" fn does_work(dw: &DoesWork) {}

#[cfg(foo)]
#[repr(C)]
pub struct DoesWork {}

#[cfg(bar)]
#[repr(C)]
pub struct DoesWork {}

#[cfg(feature = "foobar")]
pub mod foo {
    #[cfg(foo)]
    pub const DOES_NOT_WORK: i32 = 1;

    #[cfg(bar)]
    pub const DOES_NOT_WORK: i32 = 2;

    #[cfg(foo)]
    #[no_mangle]
    pub unsafe extern "C" fn does_not_work(dnw: &DoesNotWork) {}

    #[cfg(bar)]
    #[no_mangle]
    pub unsafe extern "C" fn does_not_work(dnw: &DoesNotWork) {}

    #[cfg(foo)]
    #[repr(C)]
    pub struct DoesNotWork {}

    #[cfg(bar)]
    #[repr(C)]
    pub struct DoesNotWork {}
}