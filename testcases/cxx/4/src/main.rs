extern crate cxx;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("test.h");
        type Borrowed<'a>;
        fn f(x: &CxxString) -> UniquePtr<Borrowed>;
    }
}

fn main() {}