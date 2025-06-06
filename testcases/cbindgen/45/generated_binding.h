#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t data[LEN];
} Test;
#define Test_LEN 8

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void hello(Test *test);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
