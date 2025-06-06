template <typename _Tp, typename _Alloc>
struct _Vector_base {
  union _Storage {
    constexpr _Storage() : _M_byte() {}
    ~_Storage() {}
    _Storage& operator=(const _Storage&) = delete;
    unsigned char _M_byte;
    _Tp _M_val;
  };
};