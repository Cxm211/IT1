// include/header.h

#pragma once
#include "rust/cxx.h"

inline void call_back(int32_t &i, rust::Fn<void(int32_t &)> f) {
  f(i);
}