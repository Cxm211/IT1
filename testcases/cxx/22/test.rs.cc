#include <cstdint>
#include <type_traits>

namespace shared {
  struct Color;
}

namespace shared {
#ifndef CXXBRIDGE1_STRUCT_shared$Color
#define CXXBRIDGE1_STRUCT_shared$Color
struct Color final {
  ::std::uint8_t r;
  ::std::uint8_t g;
  ::std::uint8_t b;

  bool is_white() const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_shared$Color
} // namespace shared

namespace rust_part {
extern "C" {
bool rust_part$cxxbridge1$Color$is_white(::shared::Color const &self) noexcept;
} // extern "C"

bool Color::is_white() const noexcept {
  return rust_part$cxxbridge1$Color$is_white(*this);
}
} // namespace rust_part
