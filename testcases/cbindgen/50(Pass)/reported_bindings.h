#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>


struct Field {
  int32_t a;
};

struct Foo {
  enum class Tag {
    B,
  };

  struct B_Body {
    Field 0;
  };

  Tag tag;
  union {
    B_Body b;
  };
};


extern "C" {

Foo foo();

} // extern "C"