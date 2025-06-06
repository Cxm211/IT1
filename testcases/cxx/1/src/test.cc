#include "../include/test.h"

#include "../target/cxxbridge/cxx_issue/src/main.rs.h"

#include <iostream>

::std::int32_t
CppObject::value() const
{
  return 10;
}

void
cpp_run()
{
  // Works
  {
    auto rust = new_rust_object(new CppObject());
    std::cout << rust->getter_raw()->value() << std::endl;
  }

  // doesn't work
  {
    auto rust = new_rust_object(new CppObject());
    std::cout << rust->getter_ref()->value() << std::endl;
  }
}