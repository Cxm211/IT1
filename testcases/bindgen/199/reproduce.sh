bindgen input.h -o generated_bindings.rs \
    --enable-cxx-namespaces \
    --allowlist-type "octave.*" \
    --allowlist-function "octave.*" \
    --opaque-type "octave.refcount" \
    --opaque-type "std::.*" \
    --use-core \
    -- -x c++ -std=gnu++11