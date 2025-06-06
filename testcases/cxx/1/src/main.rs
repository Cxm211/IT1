extern crate cxx;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("test.h");
        type CppObject;

        fn cpp_run();
    }

    extern "Rust" {
        type RustObject;
        unsafe fn getter_ref<'a>(self: &'a RustObject) -> &'a *mut CppObject;
        unsafe fn getter_raw(self: &RustObject) -> *mut CppObject;

        unsafe fn new_rust_object(inner: *mut CppObject) -> Box<RustObject>;
    }
}

struct RustObject {
    inner: *mut ffi::CppObject,
}

impl RustObject {
    pub fn getter_ref(&self) -> &*mut ffi::CppObject {
        &self.inner
    }

    pub fn getter_raw(&self) -> *mut ffi::CppObject {
        self.inner
    }
}

fn new_rust_object(inner: *mut ffi::CppObject) -> Box<RustObject> {
    Box::new(RustObject { inner })
}

fn main() {
    ffi::cpp_run();
}