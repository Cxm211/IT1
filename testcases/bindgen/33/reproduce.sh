bindgen input.h -o generated_bindings.rs  --opaque-type d -- -x c++ -std=c++14
bindgen input1.h -o generated_bindings1.rs --enable-cxx-namespaces --use-core -- -x c++ -std=c++14