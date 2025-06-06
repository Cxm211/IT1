

#[cxx::bridge]
mod ffi {
    struct Struct {
        vec: Vec<u8>,
    }
}