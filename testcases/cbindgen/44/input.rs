/// prefix_with_name = true
#[repr(C)]
pub enum A {
	VarA(),
	VarB(u8),
}

#[no_mangle]
pub extern "C" fn root(a: A) {
	println!("root");
	}