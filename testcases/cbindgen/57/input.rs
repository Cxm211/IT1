use libc::int32_t;
type DoFn = unsafe extern "C" fn (x: int32_t, y: int32_t) -> int32_t;
type NonNullAlias<T> = NonNull<T>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Holder {
    func: Option<DoFn>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Data {
    data: Option<NonNullAlias<int32_t>>,
}

#[no_mangle]
pub extern "C" fn root(
    holder: Holder,
    data: Data, 
) { }
