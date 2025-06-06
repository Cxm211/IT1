#[repr(C, u8)]
enum E {
    Double(f64),
    Float(f32),
}

#[repr(C, u8)]
enum F {
    double(f64),
    float(f32),
}

#[no_mangle]
pub extern "C" fn root(
    e: E,
    f: F,
    namespace: i32,
    float: f32,
) { }