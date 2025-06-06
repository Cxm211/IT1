/// The object allocated by an Arc<T>
#[repr(C)]
struct ArcInner<T: ?Sized> {
    count: atomic::AtomicUsize,
    #[cfg(feature = "track_alloc_size")]
    alloc_size: usize,
    data: T,
}
#[no_mangle]
pub extern "C" fn do_smth(t: ArcInner<u8>) {
    assert!(t == t);
}
