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
        pub struct Rooted<a> {
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<a>>,
            pub b: a,
        }
        pub type AutoValueVector_Vec = ::std::os::raw::c_int;
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of template specialization: Rooted_open0_int_close0"]
            [::std::mem::size_of::<root::JS::Rooted<::std::os::raw::c_int>>() - 4usize];
        ["Align of template specialization: Rooted_open0_int_close0"]
            [::std::mem::align_of::<root::JS::Rooted<::std::os::raw::c_int>>() - 4usize];
    };
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of template specialization: Rooted_open0_AutoValueVector_Vec_close0"]
            [::std::mem::size_of::<root::JS::Rooted<root::JS::AutoValueVector_Vec>>() - 4usize];
        ["Align of template specialization: Rooted_open0_AutoValueVector_Vec_close0"]
            [::std::mem::align_of::<root::JS::Rooted<root::JS::AutoValueVector_Vec>>() - 4usize];
    };
}
