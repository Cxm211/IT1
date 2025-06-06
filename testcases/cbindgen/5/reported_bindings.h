#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Result_i32_Tag {
  Result_i32_Ok_i32,
  Result_i32_Err_i32,
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

typedef struct Result_i32 I32Result;

I32Result hello(void);