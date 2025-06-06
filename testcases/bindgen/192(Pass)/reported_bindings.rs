/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug)]
pub struct B {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 1usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0B@@QEAA@XZ"]
    pub fn B_B(this: *mut B);
}
unsafe extern "C" {
    #[link_name = "\u{1}??DB@@QEAA@XZ"]
    pub fn B_B_destructor(this: *mut B);
}
impl B {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        B_B(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        B_B_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct C {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 1usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}??0C@@QEAA@XZ"]
    pub fn C_C(this: *mut C);
}
unsafe extern "C" {
    #[link_name = "\u{1}??DC@@QEAA@XZ"]
    pub fn C_C_destructor(this: *mut C);
}
impl C {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        C_C(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        C_C_destructor(self)
    }
}
