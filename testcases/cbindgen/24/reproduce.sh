cbindgen --config cbindgen.toml -o generated_binding.h input.rs
cbindgen --lang "C++" -o generated_binding.hpp input.rs
