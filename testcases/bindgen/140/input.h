namespace std
{
  template < typename > struct char_traits;
    template < typename _CharT, typename _Traits =
    char_traits < _CharT > >class basic_fbstring;
}
namespace __gnu_cxx
{
  template < typename > struct char_traits;
}
namespace std
{
  template < class _CharT > struct char_traits:__gnu_cxx::char_traits <
    _CharT >
  {
  };
}