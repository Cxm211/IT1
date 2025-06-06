pub type Function = fn() -> usize;

#[no_mangle]
pub extern "C" fn root(function: Function) -> usize {
    function()
}
