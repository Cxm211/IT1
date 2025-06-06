#[repr(C)]
pub struct Matrix {
    pub data: [f32; 20],
}

#[repr(C)]
pub enum Data {
    Something,
    Matrix(Matrix),
}

#[no_mangle]
pub unsafe extern "C" fn new_data() -> *mut Data {}