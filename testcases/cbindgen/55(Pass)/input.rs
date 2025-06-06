pub type Fun = fn(&str) -> Option<Vec<&str>>;
  
#[no_mangle]
pub extern fn foo(x: Fun) {
}