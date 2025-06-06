extern crate cxx;


// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("test.h");

        fn call_back(i: &mut i32, f: fn(&mut i32));
    }
}

fn main() {
    let mut i = 0i32;
    ffi::call_back(&mut i, |_| {});
}