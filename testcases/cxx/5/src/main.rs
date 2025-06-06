extern crate cxx;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("test.h");

        fn call_back<'a>(i: &'a i32, f: fn(&'a i32));
    }
}

fn main() {
    let i = 0i32;
    ffi::call_back(&i, |_| {});
}