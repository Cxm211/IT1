#[repr(C)]
pub struct ChainedStruct<'c> {
    next: Option<&'c ChainedStruct<'c>>,
    s_type: SType,
}
#[no_mangle]
pub extern fn hello(chained_struct: &mut ChainedStruct) {
    println!("hello");
}