#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


struct Field {
  int32_t a;
};

struct Foo {
  enum class Tag {
    B,
  };

  struct B_Body {
    Field _0;
  };

  Tag tag;
  union {
    B_Body b;
  };
};


extern "C" {

Foo foo();

}  // extern "C"
