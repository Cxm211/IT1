bindgen input.h --enable-cxx-namespaces --allowlist-type nsCSSValue --opaque-type 'nsRefPtrHashtable' -- -x c++ -std=c++14 
bindgen input1.h --enable-cxx-namespaces --allowlist-type StaticRefPtr --opaque-type 'JS::Rooted' -- -x c++ -std=c++14 
bindgen input2.h --enable-cxx-namespaces --allowlist-type CapturingContentInfo --opaque-type 'mozilla::Maybe' -- -x c++ -std=c++14 
