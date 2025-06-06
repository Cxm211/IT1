#include <stdint.h>

typedef union {
  double v[4];
} coord;

coord make_coord(double x, double y, double z, double t) {
  return (coord){.v = {x, y, z, t}};
}