extern crate libc;

#[no_mangle]
pub extern "C" fn allsorts_lookup_glyph_index(subtable: *mut libc::c_uchar, char_code: u32) -> u32 {
    abort_on_panic(move || unimplemented!())
}
