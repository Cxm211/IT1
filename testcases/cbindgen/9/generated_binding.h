#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct ChainedStruct {
  const struct ChainedStruct *next;
  SType s_type;
};

void hello(struct ChainedStruct *chained_struct);
