#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
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
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
#[repr(C)]
#[derive(Debug)]
pub struct bpf_raw_tracepoint_args {
    pub args: __IncompleteArrayField<::std::os::raw::c_ulonglong>,
}
#[test]
fn bindgen_test_layout_bpf_raw_tracepoint_args() {
    assert_eq!(
        ::std::mem::size_of::<bpf_raw_tracepoint_args>(),
        0usize,
        concat!("Size of: ", stringify!(bpf_raw_tracepoint_args))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_raw_tracepoint_args>(),
        8usize,
        concat!("Alignment of ", stringify!(bpf_raw_tracepoint_args))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_raw_tracepoint_args>())).args as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_raw_tracepoint_args),
            "::",
            stringify!(args)
        )
    );
}