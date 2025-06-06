pub type Bla = [u8; 12];

#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct Bla2([u8; 12]);

#[derive(Eq, PartialEq)]
#[repr(C)]
pub struct Test {
    data: Bla,
    data2: Bla2,
}

#[no_mangle]
pub extern "C" fn do_smth(t: Test) {
    assert!(t == t);
}