
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Vec3f32 {
    x: f32,
    y: f32,
    z: f32,
}

#[no_mangle]
pub extern "C" fn f(x: *const Vec3f32) -> f32 {
    x.x
}