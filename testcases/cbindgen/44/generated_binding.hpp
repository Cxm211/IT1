#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// prefix_with_name = true
struct A {
  enum class Tag {
    A_VarA,
    A_VarB,
  };

  struct A_VarA_Body {

  };

  struct A_VarB_Body {
    uint8_t _0;
  };

  Tag tag;
  union {
    A_VarA_Body var_a;
    A_VarB_Body var_b;
  };
};

extern "C" {

void root(A a);

}  // extern "C"
