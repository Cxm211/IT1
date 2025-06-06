#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo_InnerType<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
pub use self::Foo_InnerType as Bar;
extern "C" {
    #[link_name = "_Z4funcv"]
    pub fn func() -> Bar;
}