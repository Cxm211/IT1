#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct LargeFlags {
  uint64_t bits;

  constexpr explicit operator bool() const {
    return !!bits;
  }
  constexpr LargeFlags operator~() const {
    return LargeFlags { static_cast<decltype(bits)>(~bits) };
  }
  constexpr LargeFlags operator|(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits | other.bits) };
  }
  LargeFlags& operator|=(const LargeFlags& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr LargeFlags operator&(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits & other.bits) };
  }
  LargeFlags& operator&=(const LargeFlags& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr LargeFlags operator^(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits ^ other.bits) };
  }
  LargeFlags& operator^=(const LargeFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
};
/// Flag with a very large shift that usually would be narrowed.
constexpr static const LargeFlags LargeFlags_LARGE_SHIFT = LargeFlags{
  /* .bits = */ (uint64_t)(1ull << 44)
};
constexpr static const LargeFlags LargeFlags_INVERTED = LargeFlags{
  /* .bits = */ (uint64_t)~(LargeFlags_LARGE_SHIFT).bits
};

extern "C" {

void root(LargeFlags largest_flags);

}  // extern "C"
