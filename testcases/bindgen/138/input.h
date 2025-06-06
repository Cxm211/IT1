namespace std
{
  typedef char string;
    template < typename _Alloc > struct allocator_traits
  {
    typedef decltype ( _S_size_type_helper ( ( _Alloc * ) 0 ) ) __size_type;
  };
}
namespace __gnu_cxx
{
  template < typename _Alloc > struct __alloc_traits:std::allocator_traits <
    _Alloc >
  {
  };
}
namespace std
{
  template < typename, typename _Alloc > struct _Vector_base
  {
    __gnu_cxx::__alloc_traits < _Alloc > _Tp_alloc_type;
  };
  template < typename _Tp, typename _Alloc = _Tp > class vector:_Vector_base < _Tp,
    _Alloc >
  {
  };
  class LogMessage
  {
    class ostrstream
    {
      union
      {
        vector < string > *outvec_;
      };
    };
  };
}