#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Result_Error Result_Error;

typedef struct Result_Error FmtResult;

typedef struct Result_Error NoResult;

void hello(FmtResult fmt_result, NoResult no_result);
