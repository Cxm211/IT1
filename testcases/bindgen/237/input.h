namespace
{
  template < typename _CharT, typename = _CharT > class basic_ostream;
  template < typename, typename > class basic_ios
  {
  };
template < typename _CharT, typename _Traits > class basic_ostream:virtual basic_ios < _CharT,
    _Traits
    >
  {
  };
  class strstreambuf
  {
  };
  class ostrstream:basic_ostream < char >
  {
    strstreambuf _M_buf;
  };
  class LogMessage
  {
    class LogStream:ostrstream
    {
      int ctr_;
    };
  };
}