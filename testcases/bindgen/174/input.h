template <typename> class a {
  typedef int b;
  a &c(long, b);
};
template <> a<char> &a<char>::c(long, b);
template <> a<wchar_t> &a<wchar_t>::c(long, b);