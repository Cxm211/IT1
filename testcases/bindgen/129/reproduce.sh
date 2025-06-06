bindgen input.h  -- -x c++ -std=c++14 
bindgen input1.h --no-layout-tests --enable-cxx-namespaces  -o generated_bindings1.rs -- -x c++ -std=c++14