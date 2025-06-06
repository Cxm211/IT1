// bindgen-flags: --with-derive-hash --no-recursive-whitelist --allowlist-type "foo"


struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};