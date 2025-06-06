/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct _bindgen_ty_1 {
    pub decl_4_0: __IncompleteArrayField<[*mut ::std::os::raw::c_char; 4usize]>,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<_bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        0usize,
        "Size of _bindgen_ty_1"
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        "Alignment of _bindgen_ty_1"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).decl_4_0) as usize - ptr as usize },
        0usize,
        "Offset of field: _bindgen_ty_1::decl_4_0"
    );
}
impl Default for _bindgen_ty_1 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
extern "C" {
    pub static mut struct_2_0: _bindgen_ty_1;
}
