#[repr(C)]
pub enum Foo {
    B(Field)
}

#[no_mangle]
pub extern "C" fn foo() -> Foo {
    unimplemented!()
}

#[repr(C)]
pub struct Field {
    a: i32
}