#include <stdint.h>
struct GoodA {
  uint64_t f0 : 64;
  uint64_t f1 : 64;
  uint64_t f2 : 64;
  uint64_t f3 : 64;
};

struct BadA {
  // These fields are the same as `GoodA`.
  uint64_t f0 : 64;
  uint64_t f1 : 64;
  uint64_t f2 : 64;
  uint64_t f3 : 64;

  // Causes `storage` to have type `[u8; 40]`, but 40 > 32, so we are missing impls.
  uint8_t bad : 1;
};