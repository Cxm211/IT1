#define STRUCT_FIELD(name, type)                                               \
  struct name {                                                                \
    type *lh_first;                                                            \
  }

struct example {
  STRUCT_FIELD(struct1, int) field1;
};