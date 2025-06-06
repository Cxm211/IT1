#include <cstddef>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

extern "C" {
static_assert(::rust::detail::is_complete<::Type>::value, "definition of Type is required");
static_assert(sizeof(::std::unique_ptr<::Type>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Type>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Type$null(::std::unique_ptr<::Type> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Type>();
}
::Type *cxxbridge1$unique_ptr$Type$uninit(::std::unique_ptr<::Type> *ptr) noexcept {
  ::Type *uninit = reinterpret_cast<::Type *>(new ::rust::MaybeUninit<::Type>);
  ::new (ptr) ::std::unique_ptr<::Type>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$Type$raw(::std::unique_ptr<::Type> *ptr, ::Type *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Type>(raw);
}
::Type const *cxxbridge1$unique_ptr$Type$get(::std::unique_ptr<::Type> const &ptr) noexcept {
  return ptr.get();
}
::Type *cxxbridge1$unique_ptr$Type$release(::std::unique_ptr<::Type> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Type$drop(::std::unique_ptr<::Type> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Type>::value>{}(ptr);
}
} // extern "C"
