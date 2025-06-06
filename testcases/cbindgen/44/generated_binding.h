#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * prefix_with_name = true
 */
typedef enum A_Tag {
  A_VarA,
  A_VarB,
} A_Tag;

typedef struct A_VarA_Body {

} A_VarA_Body;

typedef struct A {
  A_Tag tag;
  union {
    A_VarA_Body var_a;
    struct {
      uint8_t var_b;
    };
  };
} A;

void root(struct A a);
