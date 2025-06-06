use std::fmt;

// Result<(), T> seems to trip up cbindgen, here T=fmt::Error
// as it was how I ran into it.
type FmtResult = Result<(), fmt::Error>;

type NoResult = Result<(), Error>;

#[no_mangle]
pub extern fn hello(fmt_result: FmtResult, no_result: NoResult) {
    println!("hello");
}