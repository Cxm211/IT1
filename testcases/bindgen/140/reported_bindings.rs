#[repr(C)]
#[derive(Debug, Copy)]
pub struct std_char_traits {
    pub _address: u8,
}
impl Clone for std_char_traits {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_char_traits<_CharT> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_fbstring<_CharT, _Traits> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
}