#[cxx::bridge]
mod ffi {
    #[namespace = "shared"]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    #[namespace = "rust_part"]
    extern "Rust" {
        fn is_white(self: &Color) -> bool;
    }
}