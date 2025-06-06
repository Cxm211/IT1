/// cbindgen:prefix-with-name=false
#[repr(C)]
pub enum Result<T> {
    Ok(T),
    Err(*mut Error), // Error isn't defined, doesn't matter for this case.
}

#[no_mangle]
pub extern "C" fn hello() -> Result<i32> {
    todo!()
}