#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_Wrapped Option_Wrapped;

typedef uint32_t Wrapped;

void unwrapped(uint32_t value, uint32_t opt);

void wrapped(Wrapped value, struct Option_Wrapped opt);

