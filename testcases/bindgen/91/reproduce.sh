bindgen \
    --conservative-inline-namespaces \
    --allowlist-type=C \
    --opaque-type="B" \
    input.h \
    -o generated_bindings.rs -- -x c++ --std=c++11