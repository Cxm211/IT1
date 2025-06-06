#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyType {
    _unused: [u8; 0],
}
pub type MyTypeT = MyType;