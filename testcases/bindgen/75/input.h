template <typename> struct A;
struct _Select1st;
template <typename, typename, typename, typename, typename, typename, typename,
          typename, typename, typename, bool>
struct _Map_base;
template <typename _Key, typename _Pair, typename _Alloc, typename _Equal,
          typename _H1, typename _H2, typename _Hash, typename _RehashPolicy,
          typename _Traits>
struct _Map_base<_Key, _Pair, _Alloc, _Select1st, _Equal, _H1, _H2, _Hash,
                 _RehashPolicy, _Traits, true> {
  using __hashtable_base = A<_Traits>;
  using key_type = typename __hashtable_base::key_type;
  using mapped_type = typename _Pair::type;
  mapped_type &at(const key_type &);
};
template <typename _Key, typename _Pair, typename _Alloc, typename _Equal,
          typename _H1, typename _H2, typename _Hash, typename _RehashPolicy,
          typename _Traits>
auto _Map_base<_Key, _Pair, _Alloc, _Select1st, _Equal, _H1, _H2, _Hash,
               _RehashPolicy, _Traits, true>::at(const key_type &)
  -> mapped_type & {}
