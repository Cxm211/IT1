#define int8_t char

struct FndrOpaqueInfo {
        int8_t opaque[16];
} __attribute__((aligned(2), packed));