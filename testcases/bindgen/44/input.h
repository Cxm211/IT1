#define a_warn_unused_result __attribute__((warn_unused_result))

int
must_be_used(void)
    a_warn_unused_result;