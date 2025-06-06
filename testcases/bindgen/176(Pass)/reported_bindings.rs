#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Thing {
    pub method_ptr: ::std::option::Option<unsafe extern "C" fn()>,
}