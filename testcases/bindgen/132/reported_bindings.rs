/* automatically generated by rust-bindgen */

pub type mbedtls_mpi_uint = u32;
/**
 * \brief          MPI structure
 */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _bindgen_ty_2 {
    /*!<  integer sign      */
    pub s: ::std::os::raw::c_int,
    /*!<  total # of limbs  */
    pub n: usize,
    /*!<  pointer to limbs  */
    pub p: *mut mbedtls_mpi_uint,
}
#[test]
fn bindgen_test_layout__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<_bindgen_ty_2>() , 24usize);
    assert_eq!(::std::mem::align_of::<_bindgen_ty_2>() , 8usize);
}
impl Clone for _bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
pub use self::_bindgen_ty_2 as mbedtls_mpi;