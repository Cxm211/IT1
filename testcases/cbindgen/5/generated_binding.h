#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Result_i32_Tag {
  Ok_i32,
  Err_i32,
} Result_i32_Tag;

typedef struct Result_i32 {
  Result_i32_Tag tag;
  union {
    struct {
      int32_t ok;
    };
    struct {
      Error *err;
    };
  };
} Result_i32;

struct Result_i32 hello(void);
