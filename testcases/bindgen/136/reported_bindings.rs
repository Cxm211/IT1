

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}

pub type Foo_FunctionPtr =
    ::std::option::Option<unsafe extern "C" fn() -> type-parameter-0-0>;