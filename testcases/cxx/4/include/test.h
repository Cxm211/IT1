

#pragma once
#include <memory>
#include <string>

struct Borrowed {};
std::unique_ptr<Borrowed> f(const std::string &x);