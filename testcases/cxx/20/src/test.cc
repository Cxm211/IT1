#include "../include/test.h"

#include <iostream>


namespace org {
namespace example {

void do_thing(const SharedThing& state) { std::cout << state.x << std::endl; }

}  // namespace example
}  // namespace org