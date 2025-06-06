extern crate cxx;

// lib.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust/cxx.h");
    }

    extern "Rust" {
        /// This struct comment is placed correctly.
        type SomeType;

        /// This method comment is misplaced.
        fn misplaced_comment(self: &SomeType);

        /// This free function comment is placed correctly.
        fn correctly_placed_comment();
    }
}

struct SomeType {}

impl SomeType {
    fn misplaced_comment(&self) {}
}

fn correctly_placed_comment() {}

fn main() {}