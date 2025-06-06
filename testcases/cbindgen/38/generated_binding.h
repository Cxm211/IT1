#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using Bla = uint8_t[12];

using Bla2 = uint8_t[12];

struct Test {
  Bla data;
  Bla2 data2;

  bool operator==(const Test& other) const {
    return data == other.data &&
           data2 == other.data2;
  }
};

extern "C" {

void do_smth(Test t);

}  // extern "C"
