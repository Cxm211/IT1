#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Bar;

struct Foo;

extern "C" {

void root(Bar x, Foo y);

}  // extern "C"
