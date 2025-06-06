#define POINTER_WIDTH (sizeof(void*) * 8)

struct Foo {
  int dummy;
  unsigned long foo: 1;
  unsigned long bar: POINTER_WIDTH;
};