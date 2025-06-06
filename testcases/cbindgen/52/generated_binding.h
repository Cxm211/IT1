#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if ((defined(FULL) || defined(DERIVE)) && defined(PRINTING))
typedef struct FixupContext FixupContext;
#endif

typedef struct Vec3f32 {
  float x;
  float y;
  float z;
} Vec3f32;



float f(const struct Vec3f32 *x);
