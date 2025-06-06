#pragma once
#include <string>

#include "rust/cxx.h"

namespace org {
namespace example {

struct SharedThing;

void do_thing(const SharedThing& state);

}  // namespace example
}  // namespace org