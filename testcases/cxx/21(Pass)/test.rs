

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("test.h");
        type CrossTensor;
    }

    extern "Rust" {

        /// Index a tensor using a list of tensors.
        fn index(
            tensor: &SharedPtr<CrossTensor>,
            indices: &[&SharedPtr<CrossTensor>]
        ) -> Result<SharedPtr<CrossTensor>>;
    }
}