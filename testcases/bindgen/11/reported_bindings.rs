#[repr(transparent)]
#[derive(Clone)]
pub struct Foo(pub id);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FooStruct {
    pub foo: Foo,
}