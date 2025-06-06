#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct SelfTypeTestStruct {
  uint8_t times;
} SelfTypeTestStruct;

void foo(char);

void unnamed_argument(struct SelfTypeTestStruct*);
