#[cxx::bridge(namespace = "my_usage::ffi")]
mod ffi {
    #[namespace = "zx::ffi"]
    extern "C++" {
        include!("cxx-handles-demo/src/handle.rs.h");
        type Job = crate::handle::ffi::Job;
        type Process = crate::handle::ffi::Process;
    }

    extern "Rust" {
        fn create_processes(name: &CxxString, job: &Job) -> Vec<Process>;
    }
}