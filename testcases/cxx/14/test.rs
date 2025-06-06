// str_test.rs
#[cxx::bridge]
mod ffi {
    struct A {
        a: f32,
    }

    extern "Rust"{
        fn hello(self: &A) -> String;
    }
}