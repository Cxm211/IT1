bindgen input.h -o generated_bindings.rs --enable-cxx-namespaces --allowlist-type JS::Rooted -- -x c++ -std=c++14
