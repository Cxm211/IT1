#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uintptr_t SIZE = 4;

struct WithAs;

struct WithoutAs {
  uint32_t items[SIZE];
};

extern "C" {

void some_fn(WithoutAs a, WithAs b);

}  // extern "C"
