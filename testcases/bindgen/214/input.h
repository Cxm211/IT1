#include <stdint.h>
typedef struct {
  uint16_t f6;
  uint8_t f11 : 2;
  uint8_t f12 : 4;
  uint16_t f13 : 10;
  uint8_t f14;
  uint32_t f15 : 19;
} S;