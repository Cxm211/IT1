/* automatically generated by rust-bindgen 0.71.1 */

#![cfg(not(test))]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JNINativeInterface_ {
    pub GetVersion: ::std::option::Option<
        unsafe extern "stdcall" fn(env: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub __hack: ::std::os::raw::c_ulonglong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of JNINativeInterface_"][::std::mem::size_of::<JNINativeInterface_>() - 16usize];
    ["Alignment of JNINativeInterface_"][::std::mem::align_of::<JNINativeInterface_>() - 8usize];
    ["Offset of field: JNINativeInterface_::GetVersion"]
        [::std::mem::offset_of!(JNINativeInterface_, GetVersion) - 0usize];
    ["Offset of field: JNINativeInterface_::__hack"]
        [::std::mem::offset_of!(JNINativeInterface_, __hack) - 8usize];
};
unsafe extern "stdcall" {
    #[link_name = "\u{1}?bar@@YGXXZ"]
    pub fn bar();
}
