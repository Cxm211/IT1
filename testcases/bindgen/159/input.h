#include <stdint.h>

#pragma pack(1)

typedef struct {
  uint64_t value;
} first_t;

typedef struct {
  first_t value;
} second_t;

#pragma pack()