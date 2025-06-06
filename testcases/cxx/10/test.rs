mod bindgen {
    use cxx::{type_id, ExternType};

    #[repr(C)]
    pub struct Type {}

    unsafe impl ExternType for Type {
        type Id = type_id!("Type");
        type Kind = cxx::kind::Trivial;
    }
}

#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Type = crate::bindgen::Type;
    }
    impl UniquePtr<Type> {}
}