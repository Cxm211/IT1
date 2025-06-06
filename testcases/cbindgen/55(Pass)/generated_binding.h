#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_Vec______str Option_Vec______str;

typedef struct Option_Vec______str (*Fun)(const str*);

void foo(Fun x);
