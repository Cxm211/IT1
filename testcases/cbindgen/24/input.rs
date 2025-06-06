
pub const QUOTE: char = '\'';



#[no_mangle]
pub extern "C" fn root() {
    println!("{}", QUOTE);
}
