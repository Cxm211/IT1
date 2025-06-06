#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uintptr_t (*Function)(void);

uintptr_t root(Function function);
