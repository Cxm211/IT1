bindgen --output generated_bindings.rs --enable-cxx-namespaces \
    --no-recursive-allowlist \
    --allowlist-type "octave.*" \
    --opaque-type std::string \
    input.h \
    -- -v -x c++ -std=c++11