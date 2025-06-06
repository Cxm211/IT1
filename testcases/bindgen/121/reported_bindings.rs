#[repr(C)]
#[derive(Debug, Copy)]
pub struct JNINativeInterface_ {
    pub GetVersion: ::std::option::Option<unsafe extern "C" fn(env:
                                                                   *mut ::std::os::raw::c_void)
                                              -> ::std::os::raw::c_int>,
}