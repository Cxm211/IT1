pub const POS_ONE: i8 = 1;
pub const NEG_ONE: i8 = -1;

#[no_mangle]
pub extern fn hello() {
    println!("POS_ONE is {}, NEG_ONE is {}", POS_ONE, NEG_ONE);
}