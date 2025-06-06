// bindgen-flags: -- -std=c++11

template <typename> struct A;
template <typename, typename, bool> struct _Map_base;
template <typename _Pair, typename _Traits>
struct _Map_base<_Pair, _Traits, true> {
  using __hashtable_base = A<_Traits>;
  using key_type = typename __hashtable_base::key_type;
  using mapped_type = typename _Pair::type;
  mapped_type &at(const key_type &);
};
template <typename _Pair, typename _Traits>
auto _Map_base<_Pair, _Traits, true>::at(const key_type &) -> mapped_type & {}