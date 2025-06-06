using Canvas = ::Canvas;
using Surface = ::Surface;

extern "C" {
::Canvas *cxxbridge1$Surface$canvas(::Surface &self) noexcept {
  ::Canvas &(::Surface::*canvas$)() = &::Surface::canvas;
  return &(self.*canvas$)();
}
} // extern "C"
