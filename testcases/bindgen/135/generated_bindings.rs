/* automatically generated by rust-bindgen 0.71.1 */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod JS {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Value {
            pub _address: u8,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Value"][::std::mem::size_of::<Value>() - 1usize];
            ["Alignment of Value"][::std::mem::align_of::<Value>() - 1usize];
        };
        unsafe extern "C" {
            #[link_name = "\u{1}__ZN2JS5Value1aE10JSWhyMagic"]
            pub fn Value_a(this: *mut root::JS::Value, arg1: root::JSWhyMagic);
        }
        impl Value {
            #[inline]
            pub unsafe fn a(&mut self, arg1: root::JSWhyMagic) {
                Value_a(self, arg1)
            }
        }
    }
    pub type JSWhyMagic = ::std::os::raw::c_uint;
}
