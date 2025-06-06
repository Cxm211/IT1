use std::os::raw::c_char;

#[repr(C)]
pub struct SelfTypeTestStruct {
  times: u8,
}

#[no_mangle]
pub extern "C" fn foo(_: c_char) {
    todo!()
}

#[no_mangle]
pub extern fn unnamed_argument(_: &mut SelfTypeTestStruct) {
  println!("unnamed_argument");
}