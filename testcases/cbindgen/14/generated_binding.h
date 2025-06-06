#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum A {
  A1,
  A2,
  A3,
  /**
   * Must be last for serialization purposes
   */
  Sentinel,
};
typedef uint8_t A;

enum B {
  B1,
  B2,
  B3,
  /**
   * Must be last for serialization purposes
   */
  Sentinel,
};
typedef uint8_t B;

enum C_Tag {
  C1,
  C2,
  C3,
  /**
   * Must be last for serialization purposes
   */
  Sentinel,
};
typedef uint8_t C_Tag;

typedef struct C1_Body {
  C_Tag tag;
  uint32_t a;
} C1_Body;

typedef struct C2_Body {
  C_Tag tag;
  uint32_t b;
} C2_Body;

typedef union C {
  C_Tag tag;
  C1_Body c1;
  C2_Body c2;
} C;

void root(A a, B b, union C c);
