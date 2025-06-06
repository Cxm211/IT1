#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_DoFn Option_DoFn;

typedef struct Option_NonNullAlias_i32 Option_NonNullAlias_i32;

typedef struct Holder {
  struct Option_DoFn func;
} Holder;

typedef struct Data {
  struct Option_NonNullAlias_i32 data;
} Data;

void root(struct Holder holder, struct Data data);
