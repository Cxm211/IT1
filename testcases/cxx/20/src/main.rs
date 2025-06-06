extern crate cxx;
// lib.rs

#[cxx::bridge(namespace = org::example)]
mod ffi {
    struct SharedThing {
        x: String
    }

    unsafe extern "C++" {
        include!("test.h");
        fn do_thing(state: &SharedThing);
    }
}

fn main() {
    let shared = ffi::SharedThing {
        x: "hello".to_string(),
    };
    ffi::do_thing(&shared);
}