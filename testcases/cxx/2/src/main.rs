extern crate cxx;

#[cxx::bridge]
mod ffi {
    #![deny(missing_docs)]

    unsafe extern "C++" {
        include!("test.h");

        /// wow
        fn f();
    }
}

fn main() {
    ffi::f();
}