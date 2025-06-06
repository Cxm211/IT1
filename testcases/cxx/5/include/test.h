// include/header.h

#pragma once
#include "rust/cxx.h"

inline void call_back(const int32_t &i, rust::Fn<void(const int32_t &)> f) {
  f(i);
}